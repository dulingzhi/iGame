//! Scene data — the entity/component tree stored in `scene.json`.

use serde::{Deserialize, Serialize};

/// Full scene stored as `scene.json`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SceneData {
    /// All entities in this scene.
    #[serde(default)]
    pub entities: Vec<EntityDescriptor>,
}

/// Descriptor for a single entity in the scene.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EntityDescriptor {
    /// Human-readable name (maps to Bevy `Name`).
    pub name: String,
    /// World-space transform.
    #[serde(default)]
    pub transform: TransformDescriptor,
    /// Optional list of additional component references (e.g. mesh, sprite).
    #[serde(default)]
    pub components: Vec<ComponentRef>,
}

/// Flat transform descriptor (translation / rotation / scale).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TransformDescriptor {
    /// XYZ translation.
    #[serde(default)]
    pub translation: [f32; 3],
    /// XYZW quaternion (identity default).
    #[serde(default = "default_rotation")]
    pub rotation: [f32; 4],
    /// Uniform or non-uniform scale.
    #[serde(default = "default_scale")]
    pub scale: [f32; 3],
}

fn default_rotation() -> [f32; 4] {
    [0.0, 0.0, 0.0, 1.0]
}
fn default_scale() -> [f32; 3] {
    [1.0, 1.0, 1.0]
}

impl Default for TransformDescriptor {
    fn default() -> Self {
        Self {
            translation: [0.0, 0.0, 0.0],
            rotation: default_rotation(),
            scale: default_scale(),
        }
    }
}

/// Reference to a renderable component attached to an entity.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ComponentRef {
    /// A 3-D mesh identified by a logical asset key.
    Mesh { mesh_ref: String },
    /// A 2-D sprite identified by a logical asset key.
    Sprite { texture_ref: String },
    /// A rigid-body / collider marker (physics).
    RigidBody { shape: RigidBodyShape },
}

/// Supported physics shapes.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum RigidBodyShape {
    Box,
    Sphere,
    Capsule,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_scene_json() -> &'static str {
        r#"{
  "entities": [
    {
      "name": "Ground",
      "transform": {
        "translation": [0.0, 0.0, 0.0],
        "rotation": [0.0, 0.0, 0.0, 1.0],
        "scale": [20.0, 1.0, 20.0]
      },
      "components": [
        { "type": "mesh", "mesh_ref": "plane" }
      ]
    },
    {
      "name": "Cube",
      "transform": {
        "translation": [0.0, 0.5, 0.0],
        "rotation": [0.0, 0.0, 0.0, 1.0],
        "scale": [1.0, 1.0, 1.0]
      },
      "components": [
        { "type": "mesh", "mesh_ref": "cube" }
      ]
    }
  ]
}"#
    }

    #[test]
    fn parse_scene_json() {
        let scene: SceneData = serde_json::from_str(sample_scene_json()).unwrap();
        assert_eq!(scene.entities.len(), 2);
        assert_eq!(scene.entities[0].name, "Ground");
        assert_eq!(scene.entities[1].name, "Cube");
    }

    #[test]
    fn entity_transform_values() {
        let scene: SceneData = serde_json::from_str(sample_scene_json()).unwrap();
        let ground = &scene.entities[0];
        assert_eq!(ground.transform.scale, [20.0, 1.0, 20.0]);
    }

    #[test]
    fn entity_components() {
        let scene: SceneData = serde_json::from_str(sample_scene_json()).unwrap();
        let cube = &scene.entities[1];
        assert_eq!(cube.components.len(), 1);
        assert!(matches!(
            &cube.components[0],
            ComponentRef::Mesh { mesh_ref } if mesh_ref == "cube"
        ));
    }

    #[test]
    fn transform_defaults() {
        let t = TransformDescriptor::default();
        assert_eq!(t.translation, [0.0, 0.0, 0.0]);
        assert_eq!(t.rotation, [0.0, 0.0, 0.0, 1.0]);
        assert_eq!(t.scale, [1.0, 1.0, 1.0]);
    }

    #[test]
    fn empty_scene_parses() {
        let scene: SceneData = serde_json::from_str(r#"{"entities":[]}"#).unwrap();
        assert!(scene.entities.is_empty());
    }

    #[test]
    fn scene_roundtrip() {
        let scene: SceneData = serde_json::from_str(sample_scene_json()).unwrap();
        let serialized = serde_json::to_string(&scene).unwrap();
        let deserialized: SceneData = serde_json::from_str(&serialized).unwrap();
        assert_eq!(scene, deserialized);
    }
}
