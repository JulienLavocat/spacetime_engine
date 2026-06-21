use spacetimedb::{ReducerContext, ViewContext};

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

    /// Count all entities for a specific world
    fn count(ctx: &ReducerContext, world_id: WorldId) -> usize;
}

pub trait WorldEntityView {
    /// Find an entity by its ID in a view context for a specific world
    fn find(ctx: &ViewContext, world_id: WorldId, id: u64) -> Option<Self>
    where
        Self: Sized;

    /// Get an iterator over all entities for a specific world in a view context
    fn iter(ctx: &ViewContext, world_id: WorldId) -> impl Iterator<Item = Self>;
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

    /// Count all entities
    fn count(ctx: &ReducerContext) -> u64;
}

/// A generic entity view trait for read-only views of entities.
pub trait EntityView {
    /// Find an entity by its ID in a view context
    fn find(ctx: &ViewContext, id: u64) -> Option<Self>
    where
        Self: Sized;
}
