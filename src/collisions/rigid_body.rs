use std::collections::HashMap;

use bon::{Builder, builder};
use parry3d::na::Isometry3;
use spacetimedb::{ReducerContext, SpacetimeType, Table, table};

use crate::{
    math::{Quat, Vec3},
    utils::WorldEntity,
    world,
};

pub type RigidBodyId = u64;

#[derive(SpacetimeType, Debug, Clone, Copy, PartialEq, Default)]
pub enum RigidBodyType {
    Static,
    #[default]
    Dynamic,
    Kinematic,
}

#[table(name = steng_rigid_bodies, public)]
#[derive(Builder)]
pub struct RigidBody {
    #[primary_key]
    #[auto_inc]
    #[builder(default = 0)]
    pub id: u64,
    #[index(btree)]
    #[builder(default = 1)]
    pub world_id: u64,
    #[builder(default = Vec3::ZERO)]
    pub position: Vec3,
    #[builder(default = Quat::IDENTITY)]
    pub rotation: Quat,

    #[builder(default = RigidBodyType::default())]
    pub body_type: RigidBodyType,

    pub collider_id: u64,
}

impl WorldEntity for RigidBody {
    fn insert(self, ctx: &ReducerContext) -> Self {
        ctx.db.steng_rigid_bodies().insert(self)
    }

    fn find(ctx: &ReducerContext, id: u64) -> Option<Self> {
        ctx.db.steng_rigid_bodies().id().find(id)
    }

    fn iter(ctx: &ReducerContext, world_id: world::WorldId) -> impl Iterator<Item = Self> {
        ctx.db.steng_rigid_bodies().world_id().filter(world_id)
    }

    fn as_map(ctx: &ReducerContext, world_id: world::WorldId) -> HashMap<u64, Self> {
        ctx.db
            .steng_rigid_bodies()
            .world_id()
            .filter(world_id)
            .map(|rb| (rb.id, rb))
            .collect()
    }

    fn as_vec(ctx: &ReducerContext, world_id: world::WorldId) -> Vec<Self> {
        ctx.db
            .steng_rigid_bodies()
            .world_id()
            .filter(world_id)
            .collect()
    }

    fn update(self, ctx: &ReducerContext) -> Self {
        ctx.db.steng_rigid_bodies().id().update(self)
    }

    fn delete(&self, ctx: &ReducerContext) {
        ctx.db.steng_rigid_bodies().id().delete(self.id);
    }

    fn clear(ctx: &ReducerContext, world_id: world::WorldId) {
        ctx.db
            .steng_rigid_bodies()
            .world_id()
            .filter(world_id)
            .for_each(|rb| {
                ctx.db.steng_rigid_bodies().id().delete(rb.id);
            });
    }

    fn count(ctx: &ReducerContext, world_id: world::WorldId) -> usize {
        ctx.db
            .steng_rigid_bodies()
            .world_id()
            .filter(world_id)
            .count()
    }
}

impl From<RigidBody> for Isometry3<f32> {
    fn from(value: RigidBody) -> Self {
        Isometry3::from_parts(value.position.into(), value.rotation.into())
    }
}

impl From<&RigidBody> for Isometry3<f32> {
    fn from(value: &RigidBody) -> Self {
        Isometry3::from_parts(value.position.into(), value.rotation.into())
    }
}
