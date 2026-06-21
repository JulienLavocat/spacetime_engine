use crate::world::World;
use log::info;
use spacetimedb::{ReducerContext, log_stopwatch::LogStopwatch as SpacetimeLogStopwatch};

pub struct LogStopwatch {
    event_sw: Option<SpacetimeLogStopwatch>,
    span_sw: Option<SpacetimeLogStopwatch>,
    name: String,
    should_sample: bool,
}

impl LogStopwatch {
    /// Creates a new LogStopwatch that conditionally logs timing information
    /// based on the world's debug settings.
    /// It will log the total time taken for the event, as well as
    /// allow for timing of individual spans within the event.
    /// Not every event will be logged, only a sample based on the world's
    /// debug_sample_rate.
    pub fn new(
        ctx: &ReducerContext,
        world: &World,
        name: String,
        context_debug_enabled: bool,
    ) -> Self {
        let should_sample =
            world.debug || context_debug_enabled && ctx.random::<f32>() <= world.debug_sample_rate;
        if should_sample {
            info!("--------- {name} begin ---------");
        }

        Self {
            event_sw: if should_sample {
                Some(SpacetimeLogStopwatch::new("event_time"))
            } else {
                None
            },
            span_sw: None,
            should_sample,
            name,
        }
    }

    /// Starts a new span within the event, ending any previous span.
    pub fn span(&mut self, section_name: &str) {
        if !self.should_sample {
            return;
        }

        if let Some(sw) = self.span_sw.take() {
            sw.end();
        }

        self.span_sw = Some(SpacetimeLogStopwatch::new(section_name));
    }

    /// Ends the current span, if any.
    pub fn end(&mut self) {
        if let Some(sw) = self.span_sw.take() {
            sw.end();
        }
    }
}

impl std::ops::Drop for LogStopwatch {
    fn drop(&mut self) {
        if self.should_sample {
            if let Some(sw) = self.event_sw.take() {
                sw.end();
            }
            info!("---------- {} end ----------", self.name);
        }
    }
}
