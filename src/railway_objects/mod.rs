//! The `railway_objects` module contains traits for different types of railway objects,
//! their positions within an internal model, target management, and geographical locations.
use geo_types::Coord;
use std::collections::VecDeque;

mod train;
pub use train::Train;

/// The `RailwayObject` trait represents the basic properties of a railway object,
/// including a unique identifier and a position within an internal model.
pub trait RailwayObject {
    /// Node identifiers
    type Node;

    /// Returns the unique identifier of the railway object.
    fn id(&self) -> i64;

    /// Returns the position of the railway object within the internal model.
    fn position(&self) -> Option<Self::Node>;
}

/// The `NextTarget` trait provides methods for managing a single target for a railway object.
pub trait NextTarget {
    /// Node identifiers
    type Node;

    /// Returns the next target of the railway object, if any.
    fn next_target(&self) -> Option<Self::Node>;

    /// Sets the next target of the railway object.
    fn set_next_target(&mut self, target: Option<Self::Node>);
}

/// The `MultipleTargets` trait provides methods for managing a list of targets for a railway object.
pub trait MultipleTargets {
    /// Node identifiers
    type Node;

    /// Returns the list of targets for the railway object.
    fn targets(&self) -> &VecDeque<Self::Node>;

    /// Adds a target to the list of targets for the railway object.
    fn add_target(&mut self, target: Self::Node);

    /// Removes and returns the first target from the list of targets for the railway object, if any.
    fn remove_target(&mut self) -> Option<Self::Node>;
}

/// The `GeoLocation` trait provides a method for obtaining the geographical location of a railway object.
pub trait GeoLocation {
    /// Returns the geographical location of the railway object as a coordinate.
    fn geo_location(&self) -> Option<Coord<f64>>;
}

/// The Moveable trait
pub trait Moveable {
    /// Returns the current speed of the object in km/h.
    fn speed(&self) -> f64;

    /// Sets the speed of the object in km/h.
    fn set_speed(&mut self, speed: f64);

    /// Returns the current acceleration of the object in m/s^2.
    fn acceleration(&self) -> f64;

    /// Sets the acceleration of the object in m/s^2.
    fn set_acceleration(&mut self, acceleration: f64);
}
