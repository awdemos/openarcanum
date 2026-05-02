use oa_core::{CharacterTemplate, GenerationOptions};
use oa_rules::{create_engine, get_system_definition};
use std::collections::HashMap;
use uuid::Uuid;

fn main() {
    let systems = vec!["dnd1e", "dnd2e", "dnd3e"];
    
    for system_id in systems {
        let engine = create_engine(system_id).unwrap();
        let system = get_system_definition(system_id).unwrap();
        
        let template = CharacterTemplate {
            id: Uuid::new_v4(),
            name: format!("Sample {}", system_id),
            description: "Pre-made template character".to_string(),
            system_id: system.id,
            required_attributes: system.attributes.iter().map(|a| a.name.clone()).collect(),
            optional_attributes: Vec::new(),
            default_values: HashMap::new(),
        };
        
        let options = GenerationOptions {
            use_rolled_stats: true,
            starting_level: 1,
            ..Default::default()
        };
        
        match engine.generate(&template, options) {
            Ok(result) => {
                let json = serde_json::to_string_pretty(&result.character).unwrap();
                println!("=== {} ===", system_id);
                println!("{}", json);
            }
            Err(e) => {
                eprintln!("Failed to generate {} character: {:?}", system_id, e);
            }
        }
    }
}
