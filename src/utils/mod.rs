use spacetimedb::log_stopwatch::LogStopwatch as SpacetimeLogStopwatch;

use crate::world::World;

pub struct LogStopwatch {
    sw: Option<SpacetimeLogStopwatch>,
}

impl LogStopwatch {
    pub fn new(world: &World, name: &str) -> Self {
        let sw = if world.debug {
            Some(SpacetimeLogStopwatch::new(name))
        } else {
            None
        };
        Self { sw }
    }

    pub fn end(self) {
        if let Some(sw) = self.sw {
            sw.end();
        }
    }
}
