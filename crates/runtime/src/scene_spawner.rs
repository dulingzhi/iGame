//! Spawns Bevy entities from a loaded [`MapPackage`].

use bevy::prelude::*;

use crate::{state::AppState, LoadedMap};
use igame_shared::scene::EntityData;

pub struct SceneSpawnerPlugin;

impl Plugin for SceneSpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Loading), transition_to_playing)
            .add_systems(OnEnter(AppState::Playing), spawn_map_entities);
    }
}

/// Immediately transition from Loading → Playing (no async assets for MVP).
fn transition_to_playing(mut next_state: ResMut<NextState<AppState>>) {
    next_state.set(AppState::Playing);
}

fn spawn_map_entities(mut commands: Commands, map: Res<LoadedMap>) {
    for entity_data in &map.0.scene.entities {
        spawn_entity(&mut commands, entity_data);
    }
}

fn spawn_entity(commands: &mut Commands, data: &EntityData) {
    let t = &data.transform;
    let transform = Transform {
        translation: Vec3::from_array(t.translation),
        rotation: Quat::from_array(t.rotation),
        scale: Vec3::from_array(t.scale),
    };

    let mut entity_cmds = commands.spawn(transform);

    // Name component
    if let Some(ref name) = data.name {
        entity_cmds.insert(Name::new(name.clone()));
    }

    // Sprite component (2D)
    if let Some(ref sprite_data) = data.sprite {
        let [r, g, b, a] = sprite_data.color;
        let color = Color::srgba(r, g, b, a);
        let custom_size = sprite_data.custom_size.map(|[w, h]| Vec2::new(w, h));
        entity_cmds.insert(Sprite {
            color,
            custom_size,
            ..default()
        });
    }
}
