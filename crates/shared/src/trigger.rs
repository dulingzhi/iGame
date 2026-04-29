//! Trigger graph data structures (ECA – Event / Condition / Action).

use serde::{Deserialize, Serialize};

/// A complete trigger graph stored in `triggers/<name>.json`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct TriggerGraph {
    /// All trigger nodes in this graph.
    pub nodes: Vec<TriggerNode>,
    /// All connections (edges) between node ports.
    pub edges: Vec<TriggerEdge>,
    /// Named variables scoped to this map.
    #[serde(default)]
    pub variables: Vec<Variable>,
}

/// A single node in the trigger graph.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TriggerNode {
    /// Unique node identifier within the graph.
    pub id: String,
    /// Node type key (e.g. `"event.game_start"`, `"action.spawn_unit"`).
    pub kind: String,
    /// Node-specific parameter values encoded as JSON.
    #[serde(default)]
    pub params: serde_json::Value,
}

/// A directed edge connecting an output port to an input port.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TriggerEdge {
    pub from_node: String,
    pub from_port: String,
    pub to_node: String,
    pub to_port: String,
}

/// A typed map-scoped variable.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Variable {
    pub name: String,
    pub var_type: VarType,
    pub initial_value: serde_json::Value,
}

/// Supported variable types for the trigger system.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VarType {
    Int,
    Float,
    Bool,
    String,
    Entity,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_graph_is_valid() {
        let g = TriggerGraph::default();
        assert!(g.nodes.is_empty());
        assert!(g.edges.is_empty());
    }

    #[test]
    fn trigger_graph_round_trips_json() {
        let graph = TriggerGraph {
            nodes: vec![
                TriggerNode {
                    id: "ev0".to_string(),
                    kind: "event.game_start".to_string(),
                    params: serde_json::Value::Null,
                },
                TriggerNode {
                    id: "act0".to_string(),
                    kind: "action.set_victory".to_string(),
                    params: serde_json::json!({ "team": 1 }),
                },
            ],
            edges: vec![TriggerEdge {
                from_node: "ev0".to_string(),
                from_port: "exec".to_string(),
                to_node: "act0".to_string(),
                to_port: "exec".to_string(),
            }],
            variables: vec![Variable {
                name: "kill_count".to_string(),
                var_type: VarType::Int,
                initial_value: serde_json::json!(0),
            }],
        };

        let json = serde_json::to_string_pretty(&graph).expect("serialise");
        let graph2: TriggerGraph = serde_json::from_str(&json).expect("deserialise");
        assert_eq!(graph, graph2);
    }

    #[test]
    fn var_type_serialises_as_snake_case() {
        let vt = VarType::Float;
        let s = serde_json::to_string(&vt).expect("serialise var type");
        assert_eq!(s, "\"float\"");
    }
}
