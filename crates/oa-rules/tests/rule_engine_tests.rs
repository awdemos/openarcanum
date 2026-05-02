use oa_core::{CharacterTemplate, GenerationOptions, AttributeValue};
use oa_rules::{create_engine, get_system_definition};
use uuid::Uuid;

fn make_template(system_id: Uuid) -> CharacterTemplate {
    CharacterTemplate {
        id: Uuid::new_v4(),
        name: "Test Character".to_string(),
        description: "A test character".to_string(),
        system_id,
        required_attributes: vec![
            "strength".to_string(),
            "dexterity".to_string(),
            "constitution".to_string(),
            "intelligence".to_string(),
            "wisdom".to_string(),
            "charisma".to_string(),
        ],
        optional_attributes: Vec::new(),
        default_values: std::collections::HashMap::new(),
    }
}

fn get_attribute<'a>(sheet: &'a oa_core::CharacterSheet, name: &str) -> Option<&'a AttributeValue> {
    sheet.attributes.get(name)
}

mod dnd1e_tests {
    use super::*;

    #[test]
    fn test_generate_produces_valid_character() {
        let system = get_system_definition("dnd1e").unwrap();
        let engine = create_engine("dnd1e").unwrap();
        let template = make_template(system.id);
        let options = GenerationOptions {
            use_rolled_stats: true,
            starting_level: 1,
            ..Default::default()
        };

        let result = engine.generate(&template, options).unwrap();
        assert!(!result.character.attributes.is_empty(), "Character should have attributes");
        assert!(
            get_attribute(&result.character, "strength").is_some(),
            "Character should have strength"
        );

        let validation = engine.validate(&result.character);
        assert!(validation.is_ok(), "Freshly generated character should be valid: {:?}", validation);
    }

    #[test]
    fn test_validate_rejects_impossible_stats() {
        let engine = create_engine("dnd1e").unwrap();
        let system = get_system_definition("dnd1e").unwrap();
        let template = make_template(system.id);
        let options = GenerationOptions {
            use_rolled_stats: true,
            starting_level: 1,
            ..Default::default()
        };

        let mut result = engine.generate(&template, options).unwrap();
        result.character.attributes.insert(
            "strength".to_string(),
            AttributeValue::Integer(30),
        );

        let validation = engine.validate(&result.character);
        assert!(validation.is_err(), "Character with impossible stats should fail validation");
    }

    #[test]
    fn test_level_up_succeeds() {
        let engine = create_engine("dnd1e").unwrap();
        let system = get_system_definition("dnd1e").unwrap();
        let template = make_template(system.id);
        let options = GenerationOptions {
            use_rolled_stats: true,
            starting_level: 1,
            ..Default::default()
        };

        let result = engine.generate(&template, options).unwrap();
        let mut character = result.character;

        let level_up_result = engine.apply_level_up(&mut character, Vec::new());
        assert!(level_up_result.is_ok(), "Level up should succeed: {:?}", level_up_result);
    }
}

mod dnd2e_tests {
    use super::*;

    #[test]
    fn test_generate_produces_valid_character() {
        let system = get_system_definition("dnd2e").unwrap();
        let engine = create_engine("dnd2e").unwrap();
        let template = make_template(system.id);
        let options = GenerationOptions {
            use_rolled_stats: true,
            starting_level: 1,
            ..Default::default()
        };

        let result = engine.generate(&template, options).unwrap();
        assert!(!result.character.attributes.is_empty(), "Character should have attributes");
        assert!(
            get_attribute(&result.character, "strength").is_some(),
            "Character should have strength"
        );

        let validation = engine.validate(&result.character);
        assert!(validation.is_ok(), "Freshly generated character should be valid: {:?}", validation);
    }

    #[test]
    fn test_validate_rejects_impossible_stats() {
        let engine = create_engine("dnd2e").unwrap();
        let system = get_system_definition("dnd2e").unwrap();
        let template = make_template(system.id);
        let options = GenerationOptions {
            use_rolled_stats: true,
            starting_level: 1,
            ..Default::default()
        };

        let mut result = engine.generate(&template, options).unwrap();
        result.character.attributes.insert(
            "strength".to_string(),
            AttributeValue::Integer(30),
        );

        let validation = engine.validate(&result.character);
        assert!(validation.is_err(), "Character with impossible stats should fail validation");
    }

    #[test]
    fn test_level_up_succeeds() {
        let engine = create_engine("dnd2e").unwrap();
        let system = get_system_definition("dnd2e").unwrap();
        let template = make_template(system.id);
        let options = GenerationOptions {
            use_rolled_stats: true,
            starting_level: 1,
            ..Default::default()
        };

        let result = engine.generate(&template, options).unwrap();
        let mut character = result.character;

        let level_up_result = engine.apply_level_up(&mut character, Vec::new());
        assert!(level_up_result.is_ok(), "Level up should succeed: {:?}", level_up_result);
    }
}

mod dnd3e_tests {
    use super::*;

    #[test]
    fn test_generate_produces_valid_character() {
        let system = get_system_definition("dnd3e").unwrap();
        let engine = create_engine("dnd3e").unwrap();
        let template = make_template(system.id);
        let options = GenerationOptions {
            use_rolled_stats: true,
            starting_level: 1,
            ..Default::default()
        };

        let result = engine.generate(&template, options).unwrap();
        assert!(!result.character.attributes.is_empty(), "Character should have attributes");
        assert!(
            get_attribute(&result.character, "strength").is_some(),
            "Character should have strength"
        );

        let validation = engine.validate(&result.character);
        assert!(validation.is_ok(), "Freshly generated character should be valid: {:?}", validation);
    }

    #[test]
    fn test_validate_rejects_impossible_stats() {
        let engine = create_engine("dnd3e").unwrap();
        let system = get_system_definition("dnd3e").unwrap();
        let template = make_template(system.id);
        let options = GenerationOptions {
            use_rolled_stats: true,
            starting_level: 1,
            ..Default::default()
        };

        let mut result = engine.generate(&template, options).unwrap();
        result.character.attributes.insert(
            "strength".to_string(),
            AttributeValue::Integer(30),
        );

        let validation = engine.validate(&result.character);
        assert!(validation.is_err(), "Character with impossible stats should fail validation");
    }

    #[test]
    fn test_level_up_succeeds() {
        let engine = create_engine("dnd3e").unwrap();
        let system = get_system_definition("dnd3e").unwrap();
        let template = make_template(system.id);
        let options = GenerationOptions {
            use_rolled_stats: true,
            starting_level: 1,
            ..Default::default()
        };

        let result = engine.generate(&template, options).unwrap();
        let mut character = result.character;

        let level_up_result = engine.apply_level_up(&mut character, Vec::new());
        assert!(level_up_result.is_ok(), "Level up should succeed: {:?}", level_up_result);
    }
}
