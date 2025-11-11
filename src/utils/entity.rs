use spacetimedb::ReducerContext;

use crate::world::WorldId;

/// A generic entity trait for entities that are tied to a specific world.
pub trait WorldEntity {
    /// Insert a new entity
    fn insert(self, ctx: &ReducerContext) -> Self;

    /// Find an entity by its ID
    fn find(ctx: &ReducerContext, id: u64) -> Option<Self>
    where
        Self: Sized;

    /// Get an iterator over all entities for a specific world
    fn iter(ctx: &ReducerContext, world_id: WorldId) -> impl Iterator<Item = Self>;

    /// Get all entities as a map keyed by their ID for a specific world
    fn as_map(ctx: &ReducerContext, world_id: WorldId) -> std::collections::HashMap<u64, Self>
    where
        Self: Sized;

    /// Get all entities as a vector for a specific world
    fn as_vec(ctx: &ReducerContext, world_id: WorldId) -> Vec<Self>
    where
        Self: Sized;

    /// Update an entity
    fn update(self, ctx: &ReducerContext) -> Self;

    /// Delete an entity
    fn delete(&self, ctx: &ReducerContext);

    /// Clear all entities for a specific world
    fn clear(ctx: &ReducerContext, world_id: WorldId);
}

/// A generic entity trait for entities that are not tied to a specific world.
pub trait Entity {
    /// Insert a new entity
    fn insert(self, ctx: &ReducerContext) -> Self;

    /// Find an entity by its ID
    fn find(ctx: &ReducerContext, id: u64) -> Option<Self>
    where
        Self: Sized;

    /// Get an iterator over all entities
    fn iter(ctx: &ReducerContext) -> impl Iterator<Item = Self>;

    /// Get all entities as a map keyed by their ID
    fn as_map(ctx: &ReducerContext) -> std::collections::HashMap<u64, Self>
    where
        Self: Sized;

    /// Get all entities as a vector
    fn as_vec(ctx: &ReducerContext) -> Vec<Self>
    where
        Self: Sized;

    /// Update an entity
    fn update(self, ctx: &ReducerContext) -> Self;

    /// Delete an entity
    fn delete(&self, ctx: &ReducerContext);

    /// Clear all entities
    fn clear(ctx: &ReducerContext);
}
