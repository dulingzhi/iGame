//! Application state machine.

use bevy::prelude::*;

/// Top-level application states.
#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum AppState {
    /// Loading assets and map data.
    #[default]
    Loading,
    /// In-game / playing.
    Playing,
    /// Game is paused (editor or in-game menu).
    Paused,
}

// Alias for code that was written using the old `GameState` name.
pub type GameState = AppState;
