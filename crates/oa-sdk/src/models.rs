use oa_core::{Character, GameSystem, GenerationOptions};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterList {
    pub characters: Vec<Character>,
    pub total: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemList {
    pub systems: Vec<GameSystem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateCharacterRequest {
    pub system_id: String,
    pub template_id: Option<String>,
    pub options: Option<GenerationOptions>,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentPersona {
    pub character: Character,
    pub system_id: String,
    pub behavior_guidelines: Vec<String>,
    pub voice_profile: Option<String>,
    pub knowledge_domains: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportCharacterRequest {
    pub character_id: String,
    pub format: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportCharacterResponse {
    pub format: String,
    pub content: String,
}
