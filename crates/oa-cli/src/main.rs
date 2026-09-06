use clap::{Parser, Subcommand};
use oa_core::{Character, CharacterSheet, GenerationOptions};
use oa_rules::{create_engine, get_available_systems, get_system_definition};
use oa_sdk::OpenArcanumClient;
use std::path::PathBuf;
use tracing::warn;

#[derive(Parser)]
#[command(name = "openarcanum")]
#[command(about = "Open Arcanum - RPG Character Generator Engine")]
#[command(version = env!("CARGO_PKG_VERSION"))]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(short, long, global = true, help = "Server URL for remote operations")]
    server: Option<String>,

    #[arg(short, long, global = true, help = "API key for authentication")]
    api_key: Option<String>,

    #[arg(short, long, global = true, help = "Output format (json, yaml, pretty)")]
    format: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "List available game systems")]
    Systems {
        #[arg(short, long, help = "Show detailed system information")]
        detailed: bool,
    },

    #[command(about = "Generate a new character")]
    Generate {
        #[arg(help = "Game system ID (e.g., dnd2e, generic)")]
        system: String,

        #[arg(short, long, help = "Character name")]
        name: Option<String>,

        #[arg(short, long, help = "Player name")]
        player: Option<String>,

        #[arg(short, long, help = "Starting level")]
        level: Option<u32>,

        #[arg(short, long, help = "Random seed for reproducible generation")]
        seed: Option<u64>,

        #[arg(short, long, help = "Use rolled stats instead of point buy")]
        rolled: bool,

        #[arg(short, long, help = "Point buy budget")]
        points: Option<u32>,

        #[arg(short, long, help = "Output file path")]
        output: Option<PathBuf>,

        #[arg(short, long, help = "Template ID to use")]
        template: Option<String>,
    },

    #[command(about = "Validate a character sheet")]
    Validate {
        #[arg(help = "Path to character file")]
        file: PathBuf,

        #[arg(short, long, help = "Game system ID")]
        system: String,
    },

    #[command(about = "Export character to various formats")]
    Export {
        #[arg(help = "Path to character file")]
        file: PathBuf,

        #[arg(short, long, help = "Output format (json, yaml, markdown, ogc)")]
        format: String,

        #[arg(short, long, help = "Output file path")]
        output: Option<PathBuf>,
    },

    #[command(about = "Import character from external format")]
    Import {
        #[arg(help = "Path to import file")]
        file: PathBuf,

        #[arg(short, long, help = "Import format (dndbeyond, fantasy_grounds, ogc, json)")]
        format: String,

        #[arg(short, long, help = "Game system ID to assign")]
        system: Option<String>,

        #[arg(short, long, help = "Output file path")]
        output: Option<PathBuf>,
    },

    #[command(about = "Create an agent persona from a character")]
    Persona {
        #[arg(help = "Path to character file")]
        file: PathBuf,

        #[arg(short, long, help = "Persona output file")]
        output: Option<PathBuf>,
    },

    #[command(about = "Show JSON schema for a type")]
    Schema {
        #[arg(help = "Schema type (character, system, options)")]
        schema_type: String,
    },

    #[command(about = "Start the local server")]
    Serve {
        #[arg(short, long, default_value = "3000", help = "Port to listen on")]
        port: u16,

        #[arg(short, long, help = "Host to bind to")]
        host: Option<String>,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Systems { detailed } => {
            handle_systems(detailed).await?;
        }
        Commands::Generate {
            system,
            name,
            player,
            level,
            seed,
            rolled,
            points,
            output,
            template,
        } => {
            handle_generate(system, name, player, level, seed, rolled, points, output, template, cli.server).await?;
        }
        Commands::Validate { file, system } => {
            handle_validate(file, system, cli.server).await?;
        }
        Commands::Export { file, format, output } => {
            handle_export(file, format, output).await?;
        }
        Commands::Import { file, format, system, output } => {
            handle_import(file, format, system, output).await?;
        }
        Commands::Persona { file, output } => {
            handle_persona(file, output).await?;
        }
        Commands::Schema { schema_type } => {
            handle_schema(schema_type).await?;
        }
        Commands::Serve { port, host } => {
            handle_serve(port, host).await?;
        }
    }

    Ok(())
}

async fn handle_systems(detailed: bool) -> anyhow::Result<()> {
    let systems = get_available_systems();

    if systems.is_empty() {
        println!("No game systems available.");
        return Ok(());
    }

    println!("Available game systems:");
    for system_id in systems {
        if detailed {
            match get_system_definition(system_id) {
                Ok(system) => {
                    println!("\n  {} ({})", system.name, system_id);
                    println!("    Version: {}", system.version);
                    println!("    Publisher: {}", system.publisher);
                    println!("    Levels: {}-{}", system.supported_levels.min, system.supported_levels.max);
                    println!("    Attributes: {}", system.attributes.len());
                    for attr in &system.attributes {
                        println!("      - {} ({})", attr.name, attr.abbreviation);
                    }
                }
                Err(e) => {
                    warn!("Failed to load system {}: {}", system_id, e);
                    println!("  {} ({})", system_id, system_id);
                }
            }
        } else {
            println!("  - {}", system_id);
        }
    }

    Ok(())
}

async fn handle_generate(
    system_id: String,
    name: Option<String>,
    _player: Option<String>,
    level: Option<u32>,
    seed: Option<u64>,
    rolled: bool,
    points: Option<u32>,
    output: Option<PathBuf>,
    template: Option<String>,
    server: Option<String>,
) -> anyhow::Result<()> {
    let options = GenerationOptions {
        random_seed: seed,
        point_buy_budget: points,
        use_rolled_stats: rolled,
        starting_level: level.unwrap_or(1),
        restrictions: Vec::new(),
    };

    if let Some(server_url) = server {
        let client = build_client(&server_url, None::<String>);
        let result = client
            .generate_character(&system_id, template.as_deref(), Some(options))
            .await?;

        let mut character = result.character;
        if let Some(ref n) = name {
            character.name = n.clone();
        }

        print_character(&character, output)?;
    } else {
        let engine = create_engine(&system_id)?;
        let system = get_system_definition(&system_id)?;

        let template = oa_core::CharacterTemplate {
            id: uuid::Uuid::new_v4(),
            name: template.unwrap_or_else(|| "default".to_string()),
            description: "Generated template".to_string(),
            system_id: system.id,
            required_attributes: system.attributes.iter().map(|a| a.name.clone()).collect(),
            optional_attributes: Vec::new(),
            default_values: std::collections::HashMap::new(),
        };

        let mut result = engine.generate(&template, options)?;
        if let Some(ref n) = name {
            result.character.name = n.clone();
        }

        print_character_sheet(&result, output)?;
    }

    Ok(())
}

async fn handle_validate(
    file: PathBuf,
    system_id: String,
    server: Option<String>,
) -> anyhow::Result<()> {
    let json = tokio::fs::read_to_string(&file).await?;
    let character: CharacterSheet = serde_json::from_str(&json)?;

    if let Some(server_url) = server {
        let client = build_client(&server_url, None::<String>);
        let result = client.validate_character(&system_id, &character).await?;

        if result.is_valid {
            println!("Character is valid!");
        } else {
            println!("Character has errors:");
            for error in result.errors {
                println!("  - {}", error);
            }
        }

        if !result.warnings.is_empty() {
            println!("Warnings:");
            for warning in result.warnings {
                println!("  - {}", warning);
            }
        }
    } else {
        let engine = create_engine(&system_id)?;
        let _system = get_system_definition(&system_id)?;

        match engine.validate(&character) {
            Ok(()) => println!("Character is valid!"),
            Err(errors) => {
                println!("Character has errors:");
                for error in errors {
                    println!("  - {}", error);
                }
            }
        }
    }

    Ok(())
}

async fn handle_export(
    file: PathBuf,
    format: String,
    output: Option<PathBuf>,
) -> anyhow::Result<()> {
    let json = tokio::fs::read_to_string(&file).await?;
    let character: Character = serde_json::from_str(&json)?;

    let export_format = format.parse::<oa_core::ExportFormat>()
        .map_err(|e| anyhow::anyhow!(e))?;
    let output_str = oa_core::export_character(&character, export_format)?;

    if let Some(path) = output {
        tokio::fs::write(path, output_str).await?;
        println!("Exported to file");
    } else {
        println!("{}", output_str);
    }

    Ok(())
}

async fn handle_import(
    file: PathBuf,
    format: String,
    system: Option<String>,
    output: Option<PathBuf>,
) -> anyhow::Result<()> {
    let data = tokio::fs::read_to_string(&file).await?;
    let import_format = format.parse::<oa_core::ImportFormat>()
        .map_err(|e| anyhow::anyhow!(e))?;

    let mut character = oa_core::import_character(&data, import_format)?;

    if let Some(system_id) = system {
        let system_def = get_system_definition(&system_id)?;
        character.system_id = system_def.id;
    }

    let output_str = serde_json::to_string_pretty(&character)?;

    if let Some(path) = output {
        tokio::fs::write(path, output_str).await?;
        println!("Imported character saved to file");
    } else {
        println!("{}", output_str);
    }

    Ok(())
}

async fn handle_persona(file: PathBuf, output: Option<PathBuf>) -> anyhow::Result<()> {
    let json = tokio::fs::read_to_string(&file).await?;
    let character: Character = serde_json::from_str(&json)?;

    let persona = oa_sdk::models::AgentPersona {
        character: character.clone(),
        system_id: character.system_id.to_string(),
        behavior_guidelines: generate_behavior_guidelines(&character),
        voice_profile: Some(generate_voice_profile(&character)),
        knowledge_domains: generate_knowledge_domains(&character),
    };

    let output_str = serde_json::to_string_pretty(&persona)?;

    if let Some(path) = output {
        tokio::fs::write(path, output_str).await?;
        println!("Persona exported to file");
    } else {
        println!("{}", output_str);
    }

    Ok(())
}

async fn handle_schema(schema_type: String) -> anyhow::Result<()> {
    let registry = oa_core::schema::SchemaRegistry::default_registry();
    let schema = registry
        .schemas
        .into_iter()
        .find(|s| s.name == schema_type)
        .ok_or_else(|| anyhow::anyhow!("Unknown schema type: {}. Available: character, system, generation_options", schema_type))?;

    println!("{}", serde_json::to_string_pretty(&schema)?);
    Ok(())
}

async fn handle_serve(port: u16, host: Option<String>) -> anyhow::Result<()> {
    let host = host.unwrap_or_else(|| "127.0.0.1".to_string());
    println!("Starting Open Arcanum server on {}:{}", host, port);
    println!("This feature requires the oa-server crate to be built separately.");
    println!("Run: cargo run --bin openarcanum-server");
    Ok(())
}

fn build_client(server_url: &str, api_key: Option<impl Into<String>>) -> OpenArcanumClient {
    let client = OpenArcanumClient::new(server_url);
    if let Some(key) = api_key {
        client.with_api_key(key)
    } else {
        client
    }
}

fn print_character<T: serde::Serialize>(character: &T, output: Option<PathBuf>) -> anyhow::Result<()> {
    let json = serde_json::to_string_pretty(character)?;
    if let Some(path) = output {
        std::fs::write(path, json)?;
        println!("Character saved to file");
    } else {
        println!("{}", json);
    }
    Ok(())
}

fn print_character_sheet(
    result: &oa_core::GenerationResult,
    output: Option<PathBuf>,
) -> anyhow::Result<()> {
    let json = serde_json::to_string_pretty(&result.character)?;
    if let Some(path) = output {
        std::fs::write(path, json)?;
        println!("Character saved to file");
    } else {
        println!("Generated Character:");
        println!("{}", json);

        if !result.warnings.is_empty() {
            println!("\nWarnings:");
            for warning in &result.warnings {
                println!("  - {}", warning);
            }
        }

        if !result.applied_rules.is_empty() {
            println!("\nApplied Rules:");
            for rule in &result.applied_rules {
                println!("  - {}", rule);
            }
        }
    }
    Ok(())
}

fn generate_behavior_guidelines(character: &Character) -> Vec<String> {
    let mut guidelines = vec![
        format!("Roleplay as {}.", character.identity.name),
        "Stay in character at all times.".to_string(),
        "Respond based on your character's knowledge, abilities, and personality.".to_string(),
    ];

    if !character.identity.background.is_empty() {
        guidelines.push(format!(
            "Background: {}",
            character.identity.background
        ));
    }

    if !character.identity.description.is_empty() {
        guidelines.push(format!(
            "Description: {}",
            character.identity.description
        ));
    }

    guidelines
}

fn generate_voice_profile(character: &Character) -> String {
    format!(
        "Speak as {}, a level {} character with background: {}",
        character.identity.name,
        character.level,
        character.identity.background
    )
}

fn generate_knowledge_domains(character: &Character) -> Vec<String> {
    let mut domains = Vec::new();

    for ability in &character.abilities {
        domains.push(ability.name.clone());
    }

    if !character.identity.background.is_empty() {
        domains.push(character.identity.background.clone());
    }

    domains
}
