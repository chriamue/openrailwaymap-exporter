use super::MetricsHandler;
use crate::simulation::events::{SimulationEvent, TargetReachedEvent};

/// A metrics handler that counts the number of times the target has been reached.
///
/// # Examples
///
/// ```
/// use openrailwaymap_exporter::simulation::metrics::{MetricsHandler, TargetReachedHandler};
/// use openrailwaymap_exporter::simulation::events::{RailMovableEvent, TargetReachedEvent, SimulationEvent};
///
/// let mut handler = TargetReachedHandler::new();
/// let event = TargetReachedEvent{};
///
/// handler.handle(&event);
/// assert_eq!(handler.get_value(), 1.0);
/// ```
#[derive(Default)]
pub struct TargetReachedHandler {
    target_reached_count: u64,
}

impl TargetReachedHandler {
    /// Creates a new TargetReachedHandler
    pub fn new() -> Self {
        Self {
            target_reached_count: 0,
        }
    }
}

impl MetricsHandler for TargetReachedHandler {
    fn handle(&mut self, event: &dyn SimulationEvent) {
        if let Some(_rail_movable_event) = event.as_any().downcast_ref::<TargetReachedEvent>() {
            self.target_reached_count += 1;
        }
    }

    fn get_value(&self) -> f64 {
        self.target_reached_count as f64
    }
}

#[cfg(test)]
mod tests {
    use super::{MetricsHandler, TargetReachedHandler};
    use crate::simulation::events::TargetReachedEvent;

    #[test]
    fn test_target_reached_handler() {
        let mut handler = TargetReachedHandler::new();
        let event = TargetReachedEvent {};

        handler.handle(&event);
        assert_eq!(handler.get_value(), 1.0);
    }
}
