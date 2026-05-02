use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{Character, Error, GameSystem};

#[cfg(feature = "sqlite")]
pub mod sqlite;

pub trait SystemRegistry: Send + Sync {
    fn register(&mut self, system: GameSystem) -> Result<(), Error>;
    fn get(&self, id: &str) -> Option<&GameSystem>;
    fn list(&self) -> Vec<&GameSystem>;
    fn get_by_name(&self, name: &str) -> Option<&GameSystem>;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InMemorySystemRegistry {
    systems: HashMap<String, GameSystem>,
}

impl SystemRegistry for InMemorySystemRegistry {
    fn register(&mut self, system: GameSystem) -> Result<(), Error> {
        let id = system.id.to_string();
        if self.systems.contains_key(&id) {
            return Err(Error::Validation(format!("System {} already registered", id)));
        }
        self.systems.insert(id, system);
        Ok(())
    }

    fn get(&self, id: &str) -> Option<&GameSystem> {
        self.systems.get(id)
    }

    fn list(&self) -> Vec<&GameSystem> {
        self.systems.values().collect()
    }

    fn get_by_name(&self, name: &str) -> Option<&GameSystem> {
        self.systems.values().find(|s| s.name == name)
    }
}

pub trait CharacterStore: Send + Sync {
    fn save(&mut self, character: &Character) -> Result<(), Error>;
    fn load(&self, id: &str) -> Result<Option<Character>, Error>;
    fn delete(&mut self, id: &str) -> Result<(), Error>;
    fn list(&self) -> Result<Vec<Character>, Error>;
    fn list_by_system(&self, system_id: &str) -> Result<Vec<Character>, Error>;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InMemoryCharacterStore {
    characters: HashMap<String, Character>,
}

impl CharacterStore for InMemoryCharacterStore {
    fn save(&mut self, character: &Character) -> Result<(), Error> {
        self.characters.insert(character.id.to_string(), character.clone());
        Ok(())
    }

    fn load(&self, id: &str) -> Result<Option<Character>, Error> {
        Ok(self.characters.get(id).cloned())
    }

    fn delete(&mut self, id: &str) -> Result<(), Error> {
        self.characters.remove(id);
        Ok(())
    }

    fn list(&self) -> Result<Vec<Character>, Error> {
        Ok(self.characters.values().cloned().collect())
    }

    fn list_by_system(&self, system_id: &str) -> Result<Vec<Character>, Error> {
        Ok(self
            .characters
            .values()
            .filter(|c| c.system_id.to_string() == system_id)
            .cloned()
            .collect())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SystemCapability {
    pub name: String,
    pub version: String,
    pub features: Vec<String>,
    pub endpoints: Vec<String>,
}

pub trait SystemAdapter: Send + Sync {
    fn capability(&self) -> SystemCapability;
    fn supports_system(&self, system_id: &str) -> bool;
}
