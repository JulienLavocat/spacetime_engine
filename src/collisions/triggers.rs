use bon::{Builder, builder};
use parry3d::na::Isometry3;
use spacetimedb::{ReducerContext, Table, table};

use crate::{
    collisions::rigid_body::RigidBodyId,
    math::{Quat, Vec3},
    utils::WorldEntity,
    world::WorldId,
};

pub type TriggerId = u64;

#[table(name = steng_triggers)]
#[derive(Builder)]
/// Represents a trigger volume in the world that can detect when entities enter or exit it.
/// Triggers are often used for area-based events, such as triggering animations, area of effect spells,
/// spawning enemies, or activating mechanisms when a player enters a specific area.
pub struct Trigger {
    #[builder(default = 0)]
    #[primary_key]
    #[auto_inc]
    /// The unique ID of the trigger.
    pub id: u64,
    #[index(btree)]
    #[builder(default = 1)]
    /// The world ID this trigger belongs to.
    pub world_id: u64,

    #[builder(default =Vec3::ZERO)]
    /// The position of the trigger.
    pub position: Vec3,
    #[builder(default = Quat::IDENTITY)]
    /// The rotation of the trigger.
    pub rotation: Quat,

    /// The collider associated with this trigger.
    pub collider_id: u64,

    /// The entities currently inside the trigger.
    #[builder(default = Vec::new())]
    pub entities_inside: Vec<RigidBodyId>,

    /// The entities that were added to the trigger since the last update.
    #[builder(default = Vec::new())]
    pub added_entities: Vec<RigidBodyId>,

    /// The entities that were removed from the trigger since the last update.
    #[builder(default = Vec::new())]
    pub removed_entities: Vec<RigidBodyId>,
}

impl WorldEntity for Trigger {
    fn insert(self, ctx: &ReducerContext) -> Self {
        ctx.db.steng_triggers().insert(self)
    }

    fn find(ctx: &ReducerContext, id: u64) -> Option<Self> {
        ctx.db.steng_triggers().id().find(id)
    }

    fn iter(ctx: &ReducerContext, world_id: WorldId) -> impl Iterator<Item = Self> {
        ctx.db.steng_triggers().world_id().filter(world_id)
    }

    fn as_map(ctx: &ReducerContext, world_id: WorldId) -> std::collections::HashMap<u64, Self> {
        ctx.db
            .steng_triggers()
            .world_id()
            .filter(world_id)
            .map(|trigger| (trigger.id, trigger))
            .collect()
    }

    fn as_vec(ctx: &ReducerContext, world_id: WorldId) -> Vec<Self> {
        ctx.db
            .steng_triggers()
            .world_id()
            .filter(world_id)
            .collect()
    }

    fn update(self, ctx: &ReducerContext) -> Self {
        ctx.db.steng_triggers().id().update(self)
    }

    fn delete(&self, ctx: &ReducerContext) {
        ctx.db.steng_triggers().id().delete(self.id);
    }

    fn clear(ctx: &ReducerContext, world_id: WorldId) {
        ctx.db
            .steng_triggers()
            .world_id()
            .filter(world_id)
            .for_each(|trigger| {
                ctx.db.steng_triggers().id().delete(trigger.id);
            });
    }

    fn count(ctx: &ReducerContext, world_id: WorldId) -> usize {
        ctx.db.steng_triggers().world_id().filter(world_id).count()
    }
}

impl From<Trigger> for Isometry3<f32> {
    fn from(trigger: Trigger) -> Self {
        Isometry3::from_parts(trigger.position.into(), trigger.rotation.into())
    }
}

impl From<&Trigger> for Isometry3<f32> {
    fn from(trigger: &Trigger) -> Self {
        Isometry3::from_parts(trigger.position.into(), trigger.rotation.into())
    }
}
