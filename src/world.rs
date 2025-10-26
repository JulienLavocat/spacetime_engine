use bon::{Builder, builder};

use spacetimedb::{ReducerContext, ScheduleAt, Table, table};

use crate::navigation::{self};

pub type WorldId = u64;

#[table(name = steng_world)]
#[derive(Builder, Clone, Copy)]
pub struct World {
    #[primary_key]
    #[auto_inc]
    #[builder(default = 0)]
    pub id: WorldId,
    #[builder(default = 4)]
    pub navigation_substeps: u16,
}

impl World {
    pub fn insert(self, ctx: &ReducerContext) -> Self {
        ctx.db.steng_world().insert(self)
    }
}

pub fn tick_world(ctx: &ReducerContext, world_id: WorldId, scheduled_at: ScheduleAt) {
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

    navigation::tick_navigation(ctx, world, delta_time);
}
