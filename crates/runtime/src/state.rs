use bevy::prelude::States;

/// Top-level application states.
#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum AppState {
    /// Loading assets and map data.
    #[default]
    Loading,
    /// In-game / playing.
    Playing,
}
