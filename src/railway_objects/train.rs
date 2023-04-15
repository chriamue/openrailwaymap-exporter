use std::collections::VecDeque;

use geo_types::Coord;

use super::{GeoLocation, MultipleTargets, NextTarget, RailwayObject};

/// A Train struct representing a train in the railway system.
#[derive(Default, Debug, Clone)]
pub struct Train {
    /// The unique identifier for the train.
    pub id: i64,
    /// The current position of the train, represented by a node ID.
    pub position: Option<i64>,
    /// The geographical location of the train, represented by a coordinate.
    pub geo_location: Option<Coord<f64>>,
    /// The next target node ID for the train to move towards.
    pub next_target: Option<i64>,
    /// A queue of target node IDs for the train to follow.
    pub targets: VecDeque<i64>,
}

/// Implements the `RailwayObject` trait for the `Train` struct.
impl RailwayObject for Train {
    type Node = i64;

    fn id(&self) -> i64 {
        self.id
    }

    fn position(&self) -> Option<Self::Node> {
        self.position
    }
}

/// Implements the `NextTarget` trait for the `Train` struct.
impl NextTarget for Train {
    type Node = i64;

    fn next_target(&self) -> Option<Self::Node> {
        self.next_target
    }

    fn set_next_target(&mut self, target: Option<Self::Node>) {
        self.next_target = target;
    }
}

/// Implements the `MultipleTargets` trait for the `Train` struct.
impl MultipleTargets for Train {
    type Node = i64;

    fn targets(&self) -> &VecDeque<Self::Node> {
        &self.targets
    }

    fn add_target(&mut self, target: Self::Node) {
        self.targets.push_back(target);
    }

    fn remove_target(&mut self) -> Option<Self::Node> {
        self.targets.pop_front()
    }
}

/// Implements the `GeoLocation` trait for the `Train` struct.
impl GeoLocation for Train {
    fn geo_location(&self) -> Option<Coord<f64>> {
        self.geo_location
    }
}

#[cfg(test)]
mod tests {
    use geo_types::coord;

    use super::*;

    #[test]
    fn test_train() {
        let mut train = Train {
            id: 1,
            position: Some(1),
            geo_location: Some(coord! { x:1.0, y: 2.0}),
            next_target: Some(2),
            targets: VecDeque::from(vec![2, 3, 4]),
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
}
