//! Trigger system data types — ECA (Event-Condition-Action) graphs.
//!
//! Triggers are stored in `triggers/` inside a map package as JSON files.

use serde::{Deserialize, Serialize};

/// A complete trigger graph with nodes and variable definitions.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct TriggerGraph {
    /// Named variables available to all nodes in this graph.
    #[serde(default)]
    pub variables: Vec<TriggerVariable>,
    /// Trigger nodes (each node = one ECA entry).
    #[serde(default)]
    pub nodes: Vec<TriggerNode>,
}

/// A single ECA trigger node.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TriggerNode {
    /// Unique ID within the graph.
    pub id: String,
    /// Human-readable name.
    pub name: String,
    /// Whether this trigger is currently active.
    #[serde(default = "bool_true")]
    pub enabled: bool,
    /// The event that fires this trigger.
    pub event: TriggerEvent,
    /// Optional condition (trigger only fires when this is true).
    #[serde(default)]
    pub condition: Option<TriggerCondition>,
    /// Actions executed when the trigger fires.
    #[serde(default)]
    pub actions: Vec<TriggerAction>,
}

fn bool_true() -> bool {
    true
}

/// Events that can fire a trigger.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum TriggerEvent {
    /// Fires once when the map starts.
    GameStart,
    /// Fires every `interval_secs` seconds.
    TimerPeriodic { interval_secs: f32 },
    /// Fires when an entity with matching `tag` is created.
    UnitCreated { tag: String },
    /// Fires when an entity with matching `tag` is destroyed.
    UnitDied { tag: String },
    /// Fires when an entity enters a named region.
    UnitEnterRegion { region: String },
    /// Fires when a custom event with matching `name` is raised.
    CustomEvent { name: String },
}

/// Conditions that gate trigger execution.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum TriggerCondition {
    /// `var_name op value`
    CompareInt {
        var_name: String,
        op: CompareOp,
        value: i64,
    },
    CompareFloat {
        var_name: String,
        op: CompareOp,
        value: f64,
    },
    /// Logical AND of child conditions.
    And { children: Vec<TriggerCondition> },
    /// Logical OR of child conditions.
    Or { children: Vec<TriggerCondition> },
    /// Logical NOT.
    Not { child: Box<TriggerCondition> },
    /// True with the given probability (0–1).
    RandomChance { probability: f32 },
}

/// Comparison operator.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CompareOp {
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
}

/// Actions executed when a trigger fires.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum TriggerAction {
    /// Spawn a unit at a position.
    SpawnUnit {
        unit_template: String,
        position: [f32; 3],
    },
    /// Destroy the entity that raised the event.
    DestroyEntity { entity_name: String },
    /// Set an integer variable.
    SetVarInt { var_name: String, value: i64 },
    /// Set a float variable.
    SetVarFloat { var_name: String, value: f64 },
    /// Play a sound.
    PlaySound { sound_ref: String },
    /// Play a VFX.
    PlayVfx { vfx_ref: String, position: [f32; 3] },
    /// Show a UI message.
    ShowMessage { text: String },
    /// Trigger victory.
    SetVictory,
    /// Trigger defeat.
    SetDefeat,
    /// Fire a custom event.
    FireCustomEvent { name: String },
    /// Wait (delay subsequent actions — stored as metadata).
    Wait { duration_secs: f32 },
}

/// A typed variable definition.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TriggerVariable {
    pub name: String,
    pub var_type: VarType,
    #[serde(default)]
    pub initial_value: VarValue,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum VarType {
    Int,
    Float,
    Bool,
    String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(untagged)]
pub enum VarValue {
    Int(i64),
    Float(f64),
    Bool(bool),
    Str(String),
    #[default]
    Null,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_empty_graph() {
        let json = r#"{"variables":[],"nodes":[]}"#;
        let graph: TriggerGraph = serde_json::from_str(json).unwrap();
        assert!(graph.nodes.is_empty());
    }

    #[test]
    fn parse_game_start_node() {
        let json = r#"{
            "nodes": [{
                "id": "t1",
                "name": "On Game Start",
                "event": { "type": "game_start" },
                "actions": [{ "type": "show_message", "text": "Hello!" }]
            }]
        }"#;
        let graph: TriggerGraph = serde_json::from_str(json).unwrap();
        assert_eq!(graph.nodes.len(), 1);
        assert_eq!(graph.nodes[0].event, TriggerEvent::GameStart);
    }

    #[test]
    fn parse_timer_node() {
        let json = r#"{
            "nodes": [{
                "id": "t2",
                "name": "Wave Spawner",
                "event": { "type": "timer_periodic", "interval_secs": 30.0 },
                "actions": [
                    { "type": "spawn_unit", "unit_template": "grunt", "position": [0,0,0] }
                ]
            }]
        }"#;
        let graph: TriggerGraph = serde_json::from_str(json).unwrap();
        assert!(matches!(
            graph.nodes[0].event,
            TriggerEvent::TimerPeriodic { interval_secs } if (interval_secs - 30.0).abs() < 1e-6
        ));
    }

    #[test]
    fn trigger_roundtrip() {
        let original = TriggerGraph {
            variables: vec![TriggerVariable {
                name: "kill_count".to_string(),
                var_type: VarType::Int,
                initial_value: VarValue::Int(0),
            }],
            nodes: vec![TriggerNode {
                id: "t1".to_string(),
                name: "Victory Check".to_string(),
                enabled: true,
                event: TriggerEvent::UnitDied {
                    tag: "enemy".to_string(),
                },
                condition: Some(TriggerCondition::CompareInt {
                    var_name: "kill_count".to_string(),
                    op: CompareOp::Ge,
                    value: 10,
                }),
                actions: vec![TriggerAction::SetVictory],
            }],
        };
        let json = serde_json::to_string(&original).unwrap();
        let decoded: TriggerGraph = serde_json::from_str(&json).unwrap();
        assert_eq!(original, decoded);
    }
}
