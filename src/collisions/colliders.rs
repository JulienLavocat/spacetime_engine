use spacetimedb::{SpacetimeType, Table, table};

use crate::{math::Vec3, utils::WorldEntity, world::WorldId};

pub type ColliderId = u64;

#[derive(SpacetimeType, Default, Clone, Copy, Debug, PartialEq)]
pub enum ColliderType {
    #[default]
    Sphere,
    Plane,
    Cuboid,
    Cylinder,
    Cone,
    Capsule,
    Triangle,
}

#[table(name = steng_colliders, public)]
#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct Collider {
    #[primary_key]
    #[auto_inc]
    pub id: u64,
    #[index(btree)]
    pub world_id: u64,
    pub radius: f32,
    pub normal: Vec3,
    pub height: f32,
    pub size: Vec3,
    pub point_a: Vec3,
    pub point_b: Vec3,
    pub point_c: Vec3,
    pub collider_type: ColliderType,
}

impl WorldEntity for Collider {
    fn insert(self, ctx: &spacetimedb::ReducerContext) -> Self {
        ctx.db.steng_colliders().insert(self)
    }

    fn find(ctx: &spacetimedb::ReducerContext, id: u64) -> Option<Self> {
        ctx.db.steng_colliders().id().find(id)
    }

    fn iter(ctx: &spacetimedb::ReducerContext, world_id: WorldId) -> impl Iterator<Item = Self> {
        ctx.db.steng_colliders().world_id().filter(world_id)
    }

    fn as_map(
        ctx: &spacetimedb::ReducerContext,
        world_id: WorldId,
    ) -> std::collections::HashMap<u64, Self> {
        ctx.db
            .steng_colliders()
            .world_id()
            .filter(world_id)
            .map(|collider| (collider.id, collider))
            .collect()
    }

    fn as_vec(ctx: &spacetimedb::ReducerContext, world_id: WorldId) -> Vec<Self> {
        ctx.db
            .steng_colliders()
            .world_id()
            .filter(world_id)
            .collect()
    }

    fn update(self, ctx: &spacetimedb::ReducerContext) -> Self {
        ctx.db.steng_colliders().id().update(self)
    }

    fn delete(&self, ctx: &spacetimedb::ReducerContext) {
        ctx.db.steng_colliders().id().delete(self.id);
    }

    fn clear(ctx: &spacetimedb::ReducerContext, world_id: WorldId) {
        ctx.db
            .steng_colliders()
            .world_id()
            .filter(world_id)
            .for_each(|collider| {
                ctx.db.steng_colliders().id().delete(collider.id);
            });
    }

    fn count(ctx: &spacetimedb::ReducerContext, world_id: WorldId) -> usize {
        ctx.db.steng_colliders().world_id().filter(world_id).count()
    }
}

impl Collider {
    pub fn sphere(world_id: u64, radius: f32) -> Self {
        Self {
            world_id,
            radius,
            collider_type: ColliderType::Sphere,
            ..Default::default()
        }
    }

    pub fn plane(world_id: u64, normal: Vec3) -> Self {
        Self {
            world_id,
            normal,
            collider_type: ColliderType::Plane,
            ..Default::default()
        }
    }

    pub fn cuboid(world_id: u64, size: Vec3) -> Self {
        Self {
            world_id,
            size,
            collider_type: ColliderType::Cuboid,
            ..Default::default()
        }
    }

    pub fn cylinder(world_id: u64, radius: f32, height: f32) -> Self {
        Self {
            world_id,
            radius,
            height,
            collider_type: ColliderType::Cylinder,
            ..Default::default()
        }
    }

    pub fn cone(world_id: u64, radius: f32, height: f32) -> Self {
        Self {
            world_id,
            radius,
            height,
            collider_type: ColliderType::Cone,
            ..Default::default()
        }
    }

    pub fn capsule(world_id: u64, radius: f32, height: f32) -> Self {
        Self {
            world_id,
            radius,
            height,
            collider_type: ColliderType::Capsule,
            ..Default::default()
        }
    }

    pub fn triangle(world_id: u64, point_a: Vec3, point_b: Vec3, point_c: Vec3) -> Self {
        Self {
            world_id,
            point_a,
            point_b,
            point_c,
            collider_type: ColliderType::Triangle,
            ..Default::default()
        }
    }
}
