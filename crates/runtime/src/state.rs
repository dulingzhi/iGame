//! Application state machine.

use bevy::prelude::*;

/// Top-level game states.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, States)]
pub enum GameState {
    /// Assets and map are being loaded.
    #[default]
    Loading,
    /// Map is loaded; gameplay is running.
    Playing,
    /// Game is paused (editor or in-game menu).
    Paused,
}
