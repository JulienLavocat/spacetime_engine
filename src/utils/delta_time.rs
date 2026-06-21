use spacetimedb::ScheduleAt;

pub fn get_delta_time(scheduled_at: ScheduleAt) -> f32 {
    match scheduled_at {
        ScheduleAt::Interval(dt) => dt.to_micros() as f32 / 1_000_000.0,
        _ => panic!("Expected ScheduleAt to be Interval"),
    }
}
