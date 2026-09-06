use axum::{
    extract::{Form, Path, State},
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::{get, post},
    Json, Router,
};
use tower_http::services::ServeDir;
use oa_core::{
    export::ExportFormat, schema::{SchemaInfo, SchemaRegistry}, Character, CharacterSheet,
    Error, GameSystem, GenerationOptions, GenerationResult,
};
use oa_rules::{create_engine, get_available_systems, get_system_definition};
use oa_sdk::models::{CharacterList, GenerateCharacterRequest, SystemList};
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing::info;
use uuid::Uuid;

#[derive(Clone)]
struct AppState {
    character_store: Arc<Mutex<Box<dyn oa_core::system::CharacterStore>>>,
    #[allow(dead_code)]
    system_registry: Arc<Mutex<oa_core::system::InMemorySystemRegistry>>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter("openarcanum_server=info,tower_http=debug")
        .init();

    let character_store: Box<dyn oa_core::system::CharacterStore> =
        if let Ok(db_url) = std::env::var("DATABASE_URL") {
            info!("Using SQLite database: {}", db_url);
            Box::new(oa_core::system::sqlite::SqliteCharacterStore::open(&db_url)?)
        } else {
            info!("Using in-memory character store");
            Box::new(oa_core::system::InMemoryCharacterStore::default())
        };

    let state = AppState {
        character_store: Arc::new(Mutex::new(character_store)),
        system_registry: Arc::new(Mutex::new(oa_core::system::InMemorySystemRegistry::default())),
    };

    let app = Router::new()
        .route("/health", get(health_check))
        .route("/api/v1/systems", get(list_systems))
        .route("/api/v1/systems/:id", get(get_system))
        .route("/api/v1/systems/:id/generate", post(generate_character))
        .route("/api/v1/systems/:id/validate", post(validate_character))
        .route("/api/v1/characters", post(create_character).get(list_characters))
        .route(
            "/api/v1/characters/:id",
            get(get_character).delete(delete_character),
        )
        .route("/api/v1/characters/:id/export", post(export_character))
        .route("/api/v1/schemas", get(get_schemas))
        .route("/api/v1/schemas/:name", get(get_schema))
        .route("/ui/generate", post(ui_generate_character))
        .route("/ui/characters/:id", get(ui_view_character))
        .nest_service("/static", ServeDir::new("crates/oa-server/static"))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let listener = TcpListener::bind("0.0.0.0:3000").await?;
    info!("Open Arcanum server listening on {}", listener.local_addr()?);

    axum::serve(listener, app).await?;

    Ok(())
}

async fn health_check() -> impl IntoResponse {
    "Open Arcanum server is running"
}

async fn list_systems(State(_state): State<AppState>) -> Result<Json<SystemList>, AppError> {
    let systems: Vec<GameSystem> = get_available_systems()
        .into_iter()
        .filter_map(|id| get_system_definition(id).ok())
        .collect();

    Ok(Json(SystemList { systems }))
}

async fn get_system(
    Path(system_id): Path<String>,
) -> Result<Json<GameSystem>, AppError> {
    let system = get_system_definition(&system_id)?;
    Ok(Json(system))
}

async fn generate_character(
    Path(system_id): Path<String>,
    Json(request): Json<GenerateCharacterRequest>,
) -> Result<Json<GenerationResult>, AppError> {
    let engine = create_engine(&system_id)?;
    let system = get_system_definition(&system_id)?;

    let options = request.options.unwrap_or_else(|| GenerationOptions {
        random_seed: None,
        point_buy_budget: None,
        use_rolled_stats: false,
        starting_level: 1,
        restrictions: Vec::new(),
    });

    let template = oa_core::CharacterTemplate {
        id: Uuid::new_v4(),
        name: request.name.unwrap_or_else(|| "Generated".to_string()),
        description: "Server-generated character".to_string(),
        system_id: system.id,
        required_attributes: system.attributes.iter().map(|a| a.name.clone()).collect(),
        optional_attributes: Vec::new(),
        default_values: std::collections::HashMap::new(),
    };

    let result = engine.generate(&template, options)?;
    Ok(Json(result))
}

async fn validate_character(
    Path(system_id): Path<String>,
    Json(character): Json<CharacterSheet>,
) -> Result<Json<ValidationResult>, AppError> {
    let engine = create_engine(&system_id)?;

    match engine.validate(&character) {
        Ok(()) => Ok(Json(ValidationResult {
            is_valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
        })),
        Err(errors) => Ok(Json(ValidationResult {
            is_valid: false,
            errors,
            warnings: Vec::new(),
        })),
    }
}

async fn create_character(
    State(state): State<AppState>,
    Json(character): Json<Character>,
) -> Result<(StatusCode, Json<Character>), AppError> {
    let mut store = state.character_store.lock().unwrap();
    store.save(&character)?;
    Ok((StatusCode::CREATED, Json(character)))
}

async fn get_character(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Character>, AppError> {
    let store = state.character_store.lock().unwrap();
    let character = store
        .load(&id)?
        .ok_or(Error::CharacterNotFound(id))?;
    Ok(Json(character))
}

async fn list_characters(State(state): State<AppState>) -> Result<Json<CharacterList>, AppError> {
    let store = state.character_store.lock().unwrap();
    let characters = store.list()?;
    let total = characters.len();
    Ok(Json(CharacterList { characters, total }))
}

async fn delete_character(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<StatusCode, AppError> {
    let mut store = state.character_store.lock().unwrap();
    store.delete(&id)?;
    Ok(StatusCode::NO_CONTENT)
}

async fn export_character(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(request): Json<oa_sdk::models::ExportCharacterRequest>,
) -> Result<Json<oa_sdk::models::ExportCharacterResponse>, AppError> {
    let store = state.character_store.lock().unwrap();
    let character = store
        .load(&id)?
        .ok_or(Error::CharacterNotFound(id))?;

    let format = match request.format.as_str() {
        "json" => ExportFormat::Json,
        "yaml" => ExportFormat::Yaml,
        "markdown" => ExportFormat::Markdown,
        "ogc" => ExportFormat::Ogc,
        _ => ExportFormat::Json,
    };

    let content = oa_core::export_character(&character, format)?;

    Ok(Json(oa_sdk::models::ExportCharacterResponse {
        format: request.format,
        content,
    }))
}

async fn get_schemas() -> impl IntoResponse {
    let registry = SchemaRegistry::default_registry();
    Json(registry)
}

async fn get_schema(Path(name): Path<String>) -> Result<Json<SchemaInfo>, AppError> {
    let registry = SchemaRegistry::default_registry();
    let schema = registry
        .schemas
        .into_iter()
        .find(|s| s.name == name)
        .ok_or_else(|| Error::Validation(format!("Schema '{}' not found", name)))?;
    Ok(Json(schema))
}

#[derive(Debug, Clone, serde::Deserialize)]
struct UiGenerateForm {
    system_id: String,
    name: String,
    #[serde(default)]
    use_rolled_stats: bool,
}

async fn ui_generate_character(
    Form(form): Form<UiGenerateForm>,
) -> Result<Html<String>, AppError> {
    let engine = create_engine(&form.system_id)?;
    let system = get_system_definition(&form.system_id)?;

    let options = GenerationOptions {
        random_seed: None,
        point_buy_budget: None,
        use_rolled_stats: form.use_rolled_stats,
        starting_level: 1,
        restrictions: Vec::new(),
    };

    let template = oa_core::CharacterTemplate {
        id: Uuid::new_v4(),
        name: form.name.clone(),
        description: "Web-generated character".to_string(),
        system_id: system.id,
        required_attributes: system.attributes.iter().map(|a| a.name.clone()).collect(),
        optional_attributes: Vec::new(),
        default_values: std::collections::HashMap::new(),
    };

    let result = engine.generate(&template, options)?;
    let sheet = result.character;

    let stats_html = sheet.attributes.iter().map(|(k, v)| {
        let value = match v {
            oa_core::AttributeValue::Integer(i) => i.to_string(),
            oa_core::AttributeValue::Float(f) => format!("{:.1}", f),
            oa_core::AttributeValue::String(s) => s.clone(),
            oa_core::AttributeValue::Boolean(b) => b.to_string(),
            oa_core::AttributeValue::List(_) => "[list]".to_string(),
            oa_core::AttributeValue::Map(_) => "[map]".to_string(),
        };
        format!(
            r#"<div class="stat"><div class="stat-name">{}</div><div class="stat-value">{}</div></div>"#,
            k, value
        )
    }).collect::<Vec<_>>().join("\n");

    let level = sheet.attributes.get("level").map(|v| match v {
        oa_core::AttributeValue::Integer(i) => i.to_string(),
        _ => "1".to_string(),
    }).unwrap_or_else(|| "1".to_string());

    let html = format!(
        r#"<div class="character-card">
            <h3>{}</h3>
            <p>System: {} | Level: {}</p>
            <div class="character-stats">
                {}
            </div>
        </div>"#,
        sheet.name, form.system_id, level, stats_html
    );

    Ok(Html(html))
}

async fn ui_view_character(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Html<String>, AppError> {
    let store = state.character_store.lock().unwrap();
    let character = store
        .load(&id)?
        .ok_or(Error::CharacterNotFound(id))?;

    let class = character.progression.classes.first()
        .map(|c| c.class_name.clone())
        .unwrap_or_else(|| "Unknown".to_string());

    let html = format!(
        r#"<!DOCTYPE html>
<html>
<head><title>{} - Open Arcanum</title><link rel="stylesheet" href="/static/style.css"></head>
<body>
    <header><h1>{}</h1><p>{} Level {}</p></header>
    <main>
        <section>
            <h2>Attributes</h2>
            <div class="character-stats">
                {}
            </div>
        </section>
    </main>
</body>
</html>"#,
        character.identity.name,
        character.identity.name,
        class,
        character.level,
        character.attributes.iter().map(|attr| {
            format!(
                r#"<div class="stat"><div class="stat-name">{}</div><div class="stat-value">{}</div></div>"#,
                attr.name, attr.computed_value()
            )
        }).collect::<Vec<_>>().join("\n")
    );

    Ok(Html(html))
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct ErrorResponse {
    error: String,
    code: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct ValidationResult {
    is_valid: bool,
    errors: Vec<String>,
    warnings: Vec<String>,
}

struct AppError(Error);

impl From<Error> for AppError {
    fn from(err: Error) -> Self {
        AppError(err)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_response) = match self.0 {
            Error::UnsupportedSystem(id) => (
                StatusCode::NOT_FOUND,
                ErrorResponse {
                    error: format!("System not found: {}", id),
                    code: "unsupported_system".to_string(),
                },
            ),
            Error::CharacterNotFound(id) => (
                StatusCode::NOT_FOUND,
                ErrorResponse {
                    error: format!("Character not found: {}", id),
                    code: "character_not_found".to_string(),
                },
            ),
            Error::Validation(msg) => (
                StatusCode::BAD_REQUEST,
                ErrorResponse {
                    error: msg,
                    code: "validation_error".to_string(),
                },
            ),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResponse {
                    error: self.0.to_string(),
                    code: "internal_error".to_string(),
                },
            ),
        };

        (status, Json(error_response)).into_response()
    }
}
