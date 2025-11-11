use std::collections::HashMap;

use bon::{Builder, builder};

use spacetimedb::{ReducerContext, ScheduleAt, Table, table};

use crate::navigation::{self, NavigationAgent, NavigationAgentId};

pub type WorldId = u64;

#[table(name = steng_world)]
#[derive(Builder, Clone, Copy)]
pub struct World {
    #[primary_key]
    #[auto_inc]
    #[builder(default = 0)]
    /// The unique ID of the world.
    pub id: WorldId,
    #[builder(default = false)]
    /// If true, enables debug logging and print timings for various systems.
    pub debug: bool,
    #[builder(default = debug)]
    pub debug_navigation: bool,
    #[builder(default = 0.05)]
    /// The rate at which to sample debug information, between 0.0 and 1.0.
    pub debug_sample_rate: f32,
}

impl World {
    pub fn insert(self, ctx: &ReducerContext) -> Self {
        ctx.db.steng_world().insert(self)
    }
}

pub fn tick_world(
    ctx: &ReducerContext,
    world_id: WorldId,
    scheduled_at: ScheduleAt,
    post_navigation_hook: fn(
        &ReducerContext,
        WorldId,
        f32,
        &HashMap<NavigationAgentId, NavigationAgent>,
    ),
) {
    let delta_time = match scheduled_at {
        ScheduleAt::Interval(duration) => duration.to_duration_abs().as_secs_f32(),
        _ => panic!("Expected ScheduleAt to be Interval"),
    };

    let world = ctx
        .db
        .steng_world()
        .id()
        .find(world_id)
        .expect("World not found");

    let updated_agents = navigation::tick_navigation(ctx, world, delta_time);
    post_navigation_hook(ctx, world_id, delta_time, &updated_agents);
}
