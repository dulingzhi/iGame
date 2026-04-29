//! Minimal world abstraction for the iGame runtime.

/// A lightweight game world that holds entities and their components.
///
/// This is a placeholder that will grow into a full ECS-backed world in
/// future milestones.  For now it stores entity count so the type system
/// and basic tests have something to work with.
#[derive(Debug, Default)]
pub struct World {
    entity_count: u32,
}

impl World {
    /// Create an empty world.
    pub fn new() -> Self {
        Self::default()
    }

    /// Spawn a new (empty) entity and return its id.
    pub fn spawn(&mut self) -> u32 {
        let id = self.entity_count;
        self.entity_count += 1;
        id
    }

    /// Return the total number of entities that have been spawned.
    pub fn entity_count(&self) -> u32 {
        self.entity_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_world_is_empty() {
        let world = World::new();
        assert_eq!(world.entity_count(), 0);
    }

    #[test]
    fn spawn_increments_entity_count() {
        let mut world = World::new();
        let id0 = world.spawn();
        let id1 = world.spawn();
        assert_eq!(id0, 0);
        assert_eq!(id1, 1);
        assert_eq!(world.entity_count(), 2);
    }

    #[test]
    fn spawn_returns_unique_ids() {
        let mut world = World::new();
        let ids: Vec<u32> = (0..10).map(|_| world.spawn()).collect();
        let unique: std::collections::HashSet<u32> = ids.iter().copied().collect();
        assert_eq!(unique.len(), 10);
    }
}
