use std::collections::HashMap;

use spacetimedb::{ReducerContext, Table, table};

use crate::{utils::WorldEntity, world::WorldId};

#[table(name = steng_archipelago_data)]
pub struct ArchipelagoData {
    #[primary_key]
    #[auto_inc]
    pub id: u64,
    #[index(btree)]
    pub world_id: WorldId,
    pub data: Vec<u8>,
}

impl WorldEntity for ArchipelagoData {
    fn insert(self, ctx: &ReducerContext) -> Self {
        ctx.db.steng_archipelago_data().insert(self)
    }

    fn find(ctx: &ReducerContext, id: u64) -> Option<Self> {
        ctx.db.steng_archipelago_data().id().find(id)
    }

    fn as_map(ctx: &ReducerContext, world_id: WorldId) -> HashMap<u64, Self> {
        ctx.db
            .steng_archipelago_data()
            .world_id()
            .filter(world_id)
            .map(|data| (data.id, data))
            .collect()
    }

    fn as_vec(ctx: &ReducerContext, world_id: WorldId) -> Vec<Self> {
        ctx.db
            .steng_archipelago_data()
            .world_id()
            .filter(world_id)
            .collect()
    }

    fn update(self, ctx: &ReducerContext) -> Self {
        ctx.db.steng_archipelago_data().id().update(self)
    }

    fn delete(&self, ctx: &ReducerContext) {
        ctx.db.steng_archipelago_data().id().delete(self.id);
    }

    fn clear(ctx: &ReducerContext, world_id: WorldId) {
        for data in ctx.db.steng_archipelago_data().world_id().filter(world_id) {
            ctx.db.steng_archipelago_data().id().delete(data.id);
        }
    }

    fn iter(ctx: &ReducerContext, world_id: WorldId) -> impl Iterator<Item = Self> {
        ctx.db.steng_archipelago_data().world_id().filter(world_id)
    }
}
