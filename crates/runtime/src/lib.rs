//! igame-runtime – game runtime (desktop + wasm32).
//!
//! Loads a `MapPackage`, instantiates the world, and runs the ECS schedule.

pub mod world;

pub use world::World;
