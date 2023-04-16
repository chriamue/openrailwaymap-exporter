//! The `railway_objects` module contains traits for different types of railway objects,
//! their positions within an internal model, target management, and geographical locations.
use geo_types::Coord;
use std::collections::VecDeque;

mod train;
pub use train::Train;

use crate::types::{NodeId, RailwayObjectId};

/// The `RailwayObject` trait represents the basic properties of a railway object,
/// including a unique identifier and a position within an internal model.
pub trait RailwayObject: std::fmt::Debug {
    /// Returns the unique identifier of the railway object.
    fn id(&self) -> RailwayObjectId;

    /// Returns the position of the railway object within the internal model.
    fn position(&self) -> Option<NodeId>;
    /// sets the position of the rail object
    fn set_position(&mut self, position: Option<NodeId>);
}

/// The `NextTarget` trait provides methods for managing a single target for a railway object.
pub trait NextTarget: RailwayObject {
    /// Returns the next target of the railway object, if any.
    fn next_target(&self) -> Option<NodeId>;

    /// Sets the next target of the railway object.
    fn set_next_target(&mut self, target: Option<NodeId>);
}

/// The `MultipleTargets` trait provides methods for managing a list of targets for a railway object.
pub trait MultipleTargets: RailwayObject {
    /// Returns the list of targets for the railway object.
    fn targets(&self) -> &VecDeque<NodeId>;

    /// Adds a target to the list of targets for the railway object.
    fn add_target(&mut self, target: NodeId);

    /// Removes and returns the first target from the list of targets for the railway object, if any.
    fn remove_target(&mut self) -> Option<NodeId>;
}

/// The `GeoLocation` trait provides a method for obtaining the geographical location of a railway object.
pub trait GeoLocation {
    /// Returns the geographical location of the railway object as a coordinate.
    fn geo_location(&self) -> Option<Coord<f64>>;
}

/// The Movable trait
pub trait Movable {
    /// Returns the current speed of the object in km/h.
    fn speed(&self) -> f64;

    /// Sets the speed of the object in km/h.
    fn set_speed(&mut self, speed: f64);

    /// Returns the current acceleration of the object in m/s^2.
    fn acceleration(&self) -> f64;

    /// Sets the acceleration of the object in m/s^2.
    fn set_acceleration(&mut self, acceleration: f64);
}
