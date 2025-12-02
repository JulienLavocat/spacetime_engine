use core::f32;
use std::hash::Hash;

use bon::{Builder, builder};
use spacetimedb::{ReducerContext, SpacetimeType, Table, table};

use crate::{math::Vec3, utils::WorldEntity, world::WorldId};

use super::RigidBodyId;

pub type RayCastId = u64;

#[derive(SpacetimeType, Debug, Clone)]
/// Represents a single hit result from a raycast in the physics world.
pub struct RayCastHit {
    /// The distance from the ray's origin to the hit point.
    pub distance: f32,

    /// The position in world coordinates where the ray hit.
    pub position: Vec3,

    /// The normal vector at the hit point, pointing away from the surface.
    pub normal: Vec3,

    /// The ID of the rigid body that was hit by the ray.
    pub rigid_body_id: RigidBodyId,
}

impl Hash for RayCastHit {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.distance.to_bits().hash(state);
        self.position.hash(state);
        self.normal.hash(state);
        self.rigid_body_id.hash(state);
    }
}

impl Eq for RayCastHit {}

impl PartialEq for RayCastHit {
    fn eq(&self, other: &Self) -> bool {
        self.distance.to_bits() == other.distance.to_bits()
            && self.position == other.position
            && self.normal == other.normal
            && self.rigid_body_id == other.rigid_body_id
    }
}

#[table(name = steng_raycasts)]
#[derive(Builder, Debug, Clone, PartialEq)]
pub struct RayCast {
    /// Unique identifier for the raycast.
    #[primary_key]
    #[auto_inc]
    #[builder(default = 0)]
    pub id: RayCastId,

    /// The world this raycast belongs to.
    #[index(btree)]
    #[builder(default = 1)]
    pub world_id: u64,

    /// The origin point of the raycast in world coordinates.
    pub origin: Vec3,

    /// The direction of the raycast, normalized to unit length.
    pub direction: Vec3,

    /// The maximum distance the raycast can travel.
    #[builder(default = f32::MAX)]
    pub max_distance: f32,

    /// Whether the raycast should treat shapes as solid.
    /// If `true`, rays starting inside a shape will register an immediate hit.
    /// This is typically used to detect entities fully enclosing the ray origin.
    #[builder(default = false)]
    pub solid: bool,

    /// The entities currently intersecting the raycast.
    pub hits: Vec<RayCastHit>,

    /// The hits that were added to the raycast since the last update.
    pub added_hits: Vec<RayCastHit>,

    /// The hits that were removed from the raycast since the last update.
    pub removed_hits: Vec<RayCastHit>,
}

impl RayCast {
    pub fn new(
        world_id: u64,
        origin: Vec3,
        direction: Vec3,
        max_distance: f32,
        solid: bool,
    ) -> Self {
        Self {
            id: 0,
            world_id,
            origin,
            direction: direction.normalize(),
            max_distance,
            solid,
            hits: Vec::new(),
            added_hits: Vec::new(),
            removed_hits: Vec::new(),
        }
    }
}

impl WorldEntity for RayCast {
    fn insert(self, ctx: &ReducerContext) -> Self {
        ctx.db.steng_raycasts().insert(self)
    }

    fn find(ctx: &ReducerContext, id: u64) -> Option<Self> {
        ctx.db.steng_raycasts().id().find(id)
    }

    fn iter(ctx: &ReducerContext, world_id: WorldId) -> impl Iterator<Item = Self> {
        ctx.db.steng_raycasts().world_id().filter(world_id)
    }

    fn as_map(
        ctx: &ReducerContext,
        world_id: WorldId,
    ) -> std::collections::HashMap<RayCastId, Self> {
        ctx.db
            .steng_raycasts()
            .world_id()
            .filter(world_id)
            .map(|rc| (rc.id, rc))
            .collect()
    }

    fn as_vec(ctx: &ReducerContext, world_id: WorldId) -> Vec<Self> {
        ctx.db
            .steng_raycasts()
            .world_id()
            .filter(world_id)
            .collect()
    }

    fn update(self, ctx: &ReducerContext) -> Self {
        ctx.db.steng_raycasts().id().update(self)
    }

    fn delete(&self, ctx: &ReducerContext) {
        ctx.db.steng_raycasts().id().delete(self.id);
    }

    fn clear(ctx: &ReducerContext, world_id: WorldId) {
        ctx.db
            .steng_raycasts()
            .world_id()
            .filter(world_id)
            .for_each(|rc| {
                ctx.db.steng_raycasts().id().delete(rc.id);
            });
    }

    fn count(ctx: &ReducerContext, world_id: WorldId) -> usize {
        ctx.db.steng_raycasts().world_id().filter(world_id).count()
    }
}
