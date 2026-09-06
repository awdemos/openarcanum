use crate::SdkError;
use oa_core::{Character, CharacterSheet, GameSystem, GenerationOptions, GenerationResult};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use uuid::Uuid;

const DEFAULT_TIMEOUT: Duration = Duration::from_secs(30);

#[derive(Debug, Clone)]
pub struct OpenArcanumClient {
    base_url: String,
    client: Client,
    api_key: Option<String>,
}

impl OpenArcanumClient {
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
            client: Client::builder()
                .timeout(DEFAULT_TIMEOUT)
                .build()
                .unwrap_or_default(),
            api_key: None,
        }
    }

    pub fn with_api_key(mut self, api_key: impl Into<String>) -> Self {
        self.api_key = Some(api_key.into());
        self
    }

    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.client = Client::builder()
            .timeout(timeout)
            .build()
            .unwrap_or_default();
        self
    }

    pub async fn health_check(&self) -> Result<String, SdkError> {
        let url = format!("{}/health", self.base_url);
        let response = self.client.get(&url).send().await?;
        let text = response.text().await?;
        Ok(text)
    }

    pub async fn list_systems(&self) -> Result<Vec<GameSystem>, SdkError> {
        let url = format!("{}/api/v1/systems", self.base_url);
        let response = self.request_builder(self.client.get(&url)).send().await?;
        let systems = response.json().await?;
        Ok(systems)
    }

    pub async fn get_system(&self, system_id: &str) -> Result<GameSystem, SdkError> {
        let url = format!("{}/api/v1/systems/{}", self.base_url, system_id);
        let response = self.request_builder(self.client.get(&url)).send().await?;
        let system = response.json().await?;
        Ok(system)
    }

    pub async fn generate_character(
        &self,
        system_id: &str,
        template_id: Option<&str>,
        options: Option<GenerationOptions>,
    ) -> Result<GenerationResult, SdkError> {
        let url = format!("{}/api/v1/systems/{}/generate", self.base_url, system_id);
        let request = GenerateRequest {
            template_id: template_id.map(String::from),
            options,
        };
        let response = self
            .request_builder(self.client.post(&url))
            .json(&request)
            .send()
            .await?;
        let result = response.json().await?;
        Ok(result)
    }

    pub async fn validate_character(
        &self,
        system_id: &str,
        character: &CharacterSheet,
    ) -> Result<ValidationResponse, SdkError> {
        let url = format!("{}/api/v1/systems/{}/validate", self.base_url, system_id);
        let response = self
            .request_builder(self.client.post(&url))
            .json(character)
            .send()
            .await?;
        let result = response.json().await?;
        Ok(result)
    }

    pub async fn create_character(&self, character: &Character) -> Result<Character, SdkError> {
        let url = format!("{}/api/v1/characters", self.base_url);
        let response = self
            .request_builder(self.client.post(&url))
            .json(character)
            .send()
            .await?;
        let result = response.json().await?;
        Ok(result)
    }

    pub async fn get_character(&self, character_id: Uuid) -> Result<Character, SdkError> {
        let url = format!("{}/api/v1/characters/{}", self.base_url, character_id);
        let response = self.request_builder(self.client.get(&url)).send().await?;
        let character = response.json().await?;
        Ok(character)
    }

    pub async fn list_characters(&self) -> Result<Vec<Character>, SdkError> {
        let url = format!("{}/api/v1/characters", self.base_url);
        let response = self.request_builder(self.client.get(&url)).send().await?;
        let characters = response.json().await?;
        Ok(characters)
    }

    pub async fn delete_character(&self, character_id: Uuid) -> Result<(), SdkError> {
        let url = format!("{}/api/v1/characters/{}", self.base_url, character_id);
        self.request_builder(self.client.delete(&url)).send().await?;
        Ok(())
    }

    pub async fn export_character(
        &self,
        character_id: Uuid,
        format: &str,
    ) -> Result<crate::models::ExportCharacterResponse, SdkError> {
        let url = format!("{}/api/v1/characters/{}/export", self.base_url, character_id);
        let request = crate::models::ExportCharacterRequest {
            character_id: character_id.to_string(),
            format: format.to_string(),
        };
        let response = self
            .request_builder(self.client.post(&url))
            .json(&request)
            .send()
            .await?;
        let result = response.json().await?;
        Ok(result)
    }

    fn request_builder(&self, builder: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        let builder = builder.header("Content-Type", "application/json");
        if let Some(ref key) = self.api_key {
            builder.header("X-API-Key", key)
        } else {
            builder
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateRequest {
    pub template_id: Option<String>,
    pub options: Option<GenerationOptions>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResponse {
    pub is_valid: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}
