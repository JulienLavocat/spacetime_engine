use std::collections::HashMap;

use crate::{math::Vec3, utils::WorldEntity, world::WorldId};

use spacetimedb::{ReducerContext, Table, table};

#[table(name = steng_navmesh)]
pub struct NavMesh {
    #[primary_key]
    #[auto_inc]
    pub id: u64,
    #[index(btree)]
    pub world_id: WorldId,
    pub translation: Vec3,
    pub rotation: f32,
    pub data: Vec<u8>,
}

impl WorldEntity for NavMesh {
    fn insert(self, ctx: &ReducerContext) -> Self {
        ctx.db.steng_navmesh().insert(self)
    }

    fn find(ctx: &ReducerContext, id: u64) -> Option<Self> {
        ctx.db.steng_navmesh().id().find(id)
    }

    fn as_map(ctx: &ReducerContext, world_id: WorldId) -> HashMap<u64, Self> {
        ctx.db
            .steng_navmesh()
            .world_id()
            .filter(world_id)
            .map(|nav_mesh| (nav_mesh.id, nav_mesh))
            .collect()
    }

    fn as_vec(ctx: &ReducerContext, world_id: WorldId) -> Vec<Self> {
        ctx.db.steng_navmesh().world_id().filter(world_id).collect()
    }

    fn update(self, ctx: &ReducerContext) -> Self {
        ctx.db.steng_navmesh().id().update(self)
    }

    fn delete(&self, ctx: &ReducerContext) {
        ctx.db.steng_navmesh().id().delete(self.id);
    }

    fn clear(ctx: &ReducerContext, world_id: WorldId) {
        for nav_mesh in ctx.db.steng_navmesh().world_id().filter(world_id) {
            nav_mesh.delete(ctx);
        }
    }

    fn iter(ctx: &ReducerContext, world_id: WorldId) -> impl Iterator<Item = Self> {
        ctx.db.steng_navmesh().world_id().filter(world_id)
    }

    fn count(ctx: &ReducerContext, world_id: WorldId) -> usize {
        ctx.db.steng_navmesh().world_id().filter(world_id).count()
    }
}
