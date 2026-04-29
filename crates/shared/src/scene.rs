use serde::{Deserialize, Serialize};

/// The full scene graph for a map.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MapScene {
    /// All entities that make up this scene.
    pub entities: Vec<EntityData>,
}

/// Data for a single entity in the scene.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EntityData {
    /// Optional display name for this entity.
    pub name: Option<String>,

    /// Position, rotation, and scale.
    pub transform: TransformData,

    /// Optional 2D sprite component.
    pub sprite: Option<SpriteData>,

    /// Logical tags (e.g. "unit", "ground", "player").
    pub tags: Vec<String>,
}

/// Serializable form of a 3D transform.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransformData {
    /// (x, y, z) translation.
    pub translation: [f32; 3],

    /// (x, y, z, w) quaternion rotation.
    pub rotation: [f32; 4],

    /// (x, y, z) scale factors.
    pub scale: [f32; 3],
}

impl Default for TransformData {
    fn default() -> Self {
        Self {
            translation: [0.0, 0.0, 0.0],
            rotation: [0.0, 0.0, 0.0, 1.0],
            scale: [1.0, 1.0, 1.0],
        }
    }
}

/// Serializable 2D sprite data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpriteData {
    /// RGBA color in linear [0, 1] range.
    pub color: [f32; 4],

    /// Optional explicit size in logical pixels; `None` uses the texture size.
    pub custom_size: Option<[f32; 2]>,
}
