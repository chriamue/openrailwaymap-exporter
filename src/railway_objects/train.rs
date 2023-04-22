use super::{GeoLocation, Movable, MultipleTargets, NextTarget, RailwayObject};
use crate::types::{NodeId, RailwayObjectId};
use geo::Coord;
use std::any::Any;
use std::collections::VecDeque;
use uom::si::f64::{Acceleration, Velocity};

/// A Train struct representing a train in the railway system.
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Train {
    /// The unique identifier for the train.
    pub id: i64,
    /// The current position of the train, represented by a node ID.
    pub position: Option<NodeId>,
    /// The geographical location of the train, represented by a coordinate.
    pub geo_location: Option<Coord<f64>>,
    /// The next target node ID for the train to move towards.
    pub next_target: Option<i64>,
    /// A queue of target node IDs for the train to follow.
    pub targets: VecDeque<i64>,
    /// The current speed of the train
    pub speed: Velocity,
    /// The current speed of the train
    pub max_speed: Velocity,
    /// The current acceleration of the train
    pub acceleration: Acceleration,
}

/// Implements the `RailwayObject` trait for the `Train` struct.
impl RailwayObject for Train {
    fn id(&self) -> RailwayObjectId {
        self.id
    }

    fn position(&self) -> Option<NodeId> {
        self.position
    }

    fn set_position(&mut self, position: Option<NodeId>) {
        self.position = position;
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

/// Implements the `NextTarget` trait for the `Train` struct.
impl NextTarget for Train {
    fn next_target(&self) -> Option<NodeId> {
        self.next_target
    }

    fn set_next_target(&mut self, target: Option<NodeId>) {
        self.next_target = target;
    }
}

/// Implements the `MultipleTargets` trait for the `Train` struct.
impl MultipleTargets for Train {
    fn targets(&self) -> &VecDeque<NodeId> {
        &self.targets
    }

    fn add_target(&mut self, target: NodeId) {
        self.targets.push_back(target);
    }

    fn remove_target(&mut self) -> Option<NodeId> {
        self.targets.pop_front()
    }
}

/// Implements the `GeoLocation` trait for the `Train` struct.
impl GeoLocation for Train {
    fn geo_location(&self) -> Option<Coord<f64>> {
        self.geo_location
    }

    fn set_geo_location(&mut self, location: Option<Coord<f64>>) {
        self.geo_location = location;
    }
}

impl Movable for Train {
    fn max_speed(&self) -> Velocity {
        self.max_speed
    }

    fn set_max_speed(&mut self, max_speed: Velocity) {
        self.max_speed = max_speed;
    }

    fn speed(&self) -> Velocity {
        self.speed
    }

    fn set_speed(&mut self, speed: Velocity) {
        self.speed = speed;
    }

    fn acceleration(&self) -> Acceleration {
        self.acceleration
    }

    fn set_acceleration(&mut self, acceleration: Acceleration) {
        self.acceleration = acceleration;
    }
}

#[cfg(test)]
mod tests {
    use geo::coord;
    use uom::si::{acceleration::meter_per_second_squared, velocity::kilometer_per_hour};

    use super::*;

    #[test]
    fn test_train() {
        let mut train = Train {
            id: 1,
            position: Some(1),
            geo_location: Some(coord! { x:1.0, y: 2.0}),
            next_target: Some(2),
            targets: VecDeque::from(vec![2, 3, 4]),
            ..Default::default()
        };

        assert_eq!(train.id(), 1);
        assert_eq!(train.position(), Some(1));
        assert_eq!(train.geo_location(), Some(coord! {x:1.0, y:2.0}));
        assert_eq!(train.next_target(), Some(2));
        assert_eq!(train.targets(), &VecDeque::from(vec![2, 3, 4]));

        train.set_next_target(None);
        assert_eq!(train.next_target(), None);

        train.add_target(5);
        assert_eq!(train.targets(), &VecDeque::from(vec![2, 3, 4, 5]));

        let removed_target = train.remove_target();
        assert_eq!(removed_target, Some(2));
        assert_eq!(train.targets(), &VecDeque::from(vec![3, 4, 5]));
    }

    #[test]
    fn test_train_speed() {
        let mut train = Train {
            id: 1,
            position: Some(0),
            geo_location: Some(coord! { x:1.0, y: 2.0}),
            next_target: Some(2),
            targets: VecDeque::from(vec![2, 3, 4]),
            max_speed: Velocity::new::<kilometer_per_hour>(80.0),
            speed: Velocity::new::<kilometer_per_hour>(0.0),
            acceleration: Acceleration::new::<meter_per_second_squared>(0.0),
        };

        assert_eq!(train.speed(), Velocity::new::<kilometer_per_hour>(0.0));

        train.set_speed(Velocity::new::<kilometer_per_hour>(100.0));
        assert_eq!(train.speed(), Velocity::new::<kilometer_per_hour>(100.0));
    }

    #[test]
    fn test_train_acceleration() {
        let mut train = Train {
            id: 1,
            position: Some(0),
            geo_location: Some(coord! { x:1.0, y: 2.0}),
            next_target: Some(2),
            targets: VecDeque::from(vec![2, 3, 4]),
            max_speed: Velocity::new::<kilometer_per_hour>(80.0),
            speed: Velocity::new::<kilometer_per_hour>(0.0),
            acceleration: Acceleration::new::<meter_per_second_squared>(0.0),
        };

        assert_eq!(
            train.acceleration(),
            Acceleration::new::<meter_per_second_squared>(0.0)
        );

        train.set_acceleration(Acceleration::new::<meter_per_second_squared>(1.5));
        assert_eq!(
            train.acceleration(),
            Acceleration::new::<meter_per_second_squared>(1.5)
        );
    }
}
