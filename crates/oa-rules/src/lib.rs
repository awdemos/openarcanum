pub mod dnd1e;
pub mod dnd2e;
pub mod dnd3e;

use oa_core::{Error, GameSystem, RuleEngine};

pub fn get_available_systems() -> Vec<&'static str> {
    vec!["dnd1e", "dnd2e", "dnd3e"]
}

pub fn create_engine(system_id: &str) -> Result<Box<dyn RuleEngine>, Error> {
    match system_id {
        "dnd1e" => Ok(Box::new(dnd1e::Dnd1eEngine::new())),
        "dnd2e" => Ok(Box::new(dnd2e::Dnd2eEngine::new())),
        "dnd3e" => Ok(Box::new(dnd3e::Dnd3eEngine::new())),
        _ => Err(Error::UnsupportedSystem(system_id.to_string())),
    }
}

pub fn get_system_definition(system_id: &str) -> Result<GameSystem, Error> {
    match system_id {
        "dnd1e" => Ok(dnd1e::system_definition()),
        "dnd2e" => Ok(dnd2e::system_definition()),
        "dnd3e" => Ok(dnd3e::system_definition()),
        _ => Err(Error::UnsupportedSystem(system_id.to_string())),
    }
}
