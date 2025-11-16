use std::collections::HashMap;

use bon::{Builder, builder};

use spacetimedb::{ReducerContext, ScheduleAt, Table, table};

use crate::{
    collisions,
    navigation::{self, NavigationAgent, NavigationAgentId},
    utils::{Entity, get_delta_time},
};

pub type WorldId = u64;

#[table(name = steng_world)]
#[derive(Builder, Clone, Copy)]
pub struct World {
    #[primary_key]
    #[auto_inc]
    #[builder(default = 0)]
    /// The unique ID of the world.
    pub id: WorldId,

    /// The factor by which to expand the AABB during broad-phase collision detection.
    #[builder(default = 0.0)]
    pub aabb_dilation_factor: f32,

    #[builder(default = false)]
    /// If true, enables debug logging and print timings for various systems.
    pub debug: bool,
    #[builder(default = debug)]
    pub debug_navigation: bool,
    #[builder(default = debug)]
    pub debug_behavior_trees: bool,
    #[builder(default = debug)]
    pub debug_collisions: bool,
    #[builder(default = 0.05)]
    /// The rate at which to sample debug information, between 0.0 and 1.0.
    pub debug_sample_rate: f32,
}

impl Entity for World {
    fn insert(self, ctx: &ReducerContext) -> Self {
        ctx.db.steng_world().insert(self)
    }

    fn find(ctx: &ReducerContext, id: u64) -> Option<Self> {
        ctx.db.steng_world().id().find(id)
    }

    fn iter(ctx: &ReducerContext) -> impl Iterator<Item = Self> {
        ctx.db.steng_world().iter()
    }

    fn as_map(ctx: &ReducerContext) -> std::collections::HashMap<u64, Self> {
        ctx.db
            .steng_world()
            .iter()
            .map(|world| (world.id, world))
            .collect()
    }

    fn as_vec(ctx: &ReducerContext) -> Vec<Self> {
        ctx.db.steng_world().iter().collect()
    }

    fn update(self, ctx: &ReducerContext) -> Self {
        ctx.db.steng_world().id().update(self)
    }

    fn delete(&self, ctx: &ReducerContext) {
        ctx.db.steng_world().id().delete(self.id);
    }

    fn clear(ctx: &ReducerContext) {
        ctx.db.steng_world().iter().for_each(|world| {
            world.delete(ctx);
        });
    }

    fn count(ctx: &ReducerContext) -> u64 {
        ctx.db.steng_world().count()
    }
}

pub fn tick_world(
    ctx: &ReducerContext,
    world_id: WorldId,
    scheduled_at: ScheduleAt,
    characters: impl Iterator<Item = navigation::Character>,
) -> HashMap<NavigationAgentId, NavigationAgent> {
    let delta_time = get_delta_time(scheduled_at);

    let world = World::find(ctx, world_id).expect("World not found");

    let agents = navigation::tick_navigation(ctx, &world, delta_time, characters);
    collisions::tick_collisions(ctx, &world);

    agents
}
