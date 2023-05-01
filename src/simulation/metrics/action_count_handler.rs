use crate::simulation::events::{RailMovableEvent, SimulationEvent};
use crate::simulation::metrics::MetricsHandler;
use std::any::Any;
use std::collections::HashMap;

/// `ActionCountHandler` is a metrics handler that counts the number of actions
/// performed by railway objects in a simulation.
///
/// It maintains a count of actions by their type and provides the total number
/// of actions performed as the metric value.
#[derive(Default)]
pub struct ActionCountHandler {
    action_counts: HashMap<String, u64>,
}

impl ActionCountHandler {
    /// Creates a new `ActionCountHandler`.
    ///
    /// # Returns
    ///
    /// A new `ActionCountHandler` instance.
    pub fn new() -> Self {
        Self {
            action_counts: HashMap::new(),
        }
    }
}

impl MetricsHandler for ActionCountHandler {
    fn handle(&mut self, event: &dyn SimulationEvent) {
        if let Some(rail_movable_event) = event.as_any().downcast_ref::<RailMovableEvent>() {
            let count = self
                .action_counts
                .entry(rail_movable_event.action.to_string())
                .or_insert(0);
            *count += 1;
        }
    }

    fn get_value(&self) -> f64 {
        self.action_counts.values().map(|count| *count as f64).sum()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::simulation::agents::RailMovableAction;
    use crate::simulation::events::RailMovableEvent;
    use crate::simulation::metrics::{ActionCountHandler, MetricsHandler};

    #[test]
    fn test_action_count_handler() {
        let mut handler = ActionCountHandler::new();
        let event1 = RailMovableEvent {
            action: RailMovableAction::Stop,
        };
        let event2 = RailMovableEvent {
            action: RailMovableAction::AccelerateForward { acceleration: 1 },
        };
        let event3 = RailMovableEvent {
            action: RailMovableAction::AccelerateBackward { acceleration: 1 },
        };

        handler.handle(&event1);
        handler.handle(&event2);
        handler.handle(&event3);

        assert_eq!(handler.get_value(), 3.0);
    }
}
