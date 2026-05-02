use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use crate::{AttributeValue, CharacterSheet, Error};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct RuleSet {
    pub id: Uuid,
    pub name: String,
    pub version: String,
    pub priority: u32,
    pub rules: Vec<Rule>,
    pub conditions: Vec<RuleCondition>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct Rule {
    pub id: Uuid,
    pub name: String,
    pub rule_type: RuleType,
    pub target: RuleTarget,
    pub action: RuleAction,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RuleType {
    AttributeCalculation,
    Prerequisite,
    Restriction,
    Bonus,
    Penalty,
    Transformation,
    Trigger,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct RuleTarget {
    pub target_type: TargetType,
    pub selector: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TargetType {
    Attribute,
    Ability,
    Skill,
    Feat,
    Class,
    Race,
    Item,
    Character,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum RuleAction {
    SetValue { value: AttributeValue },
    ModifyValue { operation: MathOperation, operand: i32 },
    GrantAbility { ability_id: Uuid },
    RequireChoice { options: Vec<String>, count: u32 },
    BlockAction { action: String },
    ApplyCondition { condition: String },
    ChainRule { rule_id: Uuid },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum MathOperation {
    Add,
    Subtract,
    Multiply,
    Divide,
    Set,
    Min,
    Max,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct RuleCondition {
    pub id: Uuid,
    pub condition_type: ConditionType,
    pub parameters: HashMap<String, String>,
    pub negate: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ConditionType {
    HasAttribute,
    HasAbility,
    HasFeat,
    HasClass,
    HasRace,
    AttributeValue,
    LevelRange,
    Coincident,
    Custom,
}

pub trait RuleProcessor {
    fn process(&self, character: &mut CharacterSheet, rules: &[Rule]) -> Result<RuleResult, Error>;
    fn evaluate_condition(&self, character: &CharacterSheet, condition: &RuleCondition) -> bool;
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "snake_case")]
pub struct RuleResult {
    pub applied_rules: Vec<Uuid>,
    pub modified_attributes: Vec<String>,
    pub granted_abilities: Vec<Uuid>,
    pub warnings: Vec<String>,
    pub blocked_actions: Vec<String>,
}

pub struct SimpleRuleProcessor;

impl RuleProcessor for SimpleRuleProcessor {
    fn process(&self, character: &mut CharacterSheet, rules: &[Rule]) -> Result<RuleResult, Error> {
        let mut result = RuleResult::default();

        for rule in rules {
            match &rule.action {
                RuleAction::SetValue { value } => {
                    let key = rule.target.selector.clone();
                    let json_value = serde_json::to_value(value)?;
                    character
                        .attributes
                        .insert(key.clone(), serde_json::from_value(json_value)?);
                    result.modified_attributes.push(key);
                    result.applied_rules.push(rule.id);
                }
                RuleAction::ModifyValue { operation, operand } => {
                    let key = &rule.target.selector;
                    if let Some(AttributeValue::Integer(current)) = character.attributes.get(key) {
                        let new_value = match operation {
                            MathOperation::Add => current + *operand,
                            MathOperation::Subtract => current - *operand,
                            MathOperation::Multiply => current * *operand,
                            MathOperation::Divide => current / *operand.max(&1),
                            MathOperation::Set => *operand,
                            MathOperation::Min => (*current).min(*operand),
                            MathOperation::Max => (*current).max(*operand),
                        };
                        character
                            .attributes
                            .insert(key.clone(), AttributeValue::Integer(new_value));
                        result.modified_attributes.push(key.clone());
                        result.applied_rules.push(rule.id);
                    }
                }
                RuleAction::GrantAbility { ability_id } => {
                    result.granted_abilities.push(*ability_id);
                    result.applied_rules.push(rule.id);
                }
                RuleAction::BlockAction { action } => {
                    result.blocked_actions.push(action.clone());
                    result.applied_rules.push(rule.id);
                }
                _ => {
                    result.warnings.push(format!("Unprocessed rule: {}", rule.name));
                }
            }
        }

        Ok(result)
    }

    fn evaluate_condition(&self, character: &CharacterSheet, condition: &RuleCondition) -> bool {
        let result = match condition.condition_type {
            ConditionType::HasAttribute => character.attributes.contains_key(&condition.parameters["attribute"]),
            ConditionType::AttributeValue => {
                if let Some(AttributeValue::Integer(val)) = character.attributes.get(&condition.parameters["attribute"]) {
                    let target: i32 = condition.parameters["value"].parse().unwrap_or(0);
                    val >= &target
                } else {
                    false
                }
            }
            ConditionType::LevelRange => {
                if let Some(AttributeValue::Integer(level)) = character.attributes.get("level") {
                    let min: i32 = condition.parameters["min"].parse().unwrap_or(1);
                    let max: i32 = condition.parameters["max"].parse().unwrap_or(20);
                    level >= &min && level <= &max
                } else {
                    false
                }
            }
            _ => true,
        };

        if condition.negate {
            !result
        } else {
            result
        }
    }
}
