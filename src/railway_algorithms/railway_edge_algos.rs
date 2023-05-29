use geo::{Coord, HaversineDistance, Point};
use uom::si::{f64::Length, length::meter};

use crate::{
    algorithms::{points_in_front, Distance},
    prelude::RailwayEdge,
};

/// Algorithms for railway edges.
pub trait RailwayEdgeAlgos {
    /// Calculates the distance between the current location and the last coordinate in the linestring.
    ///
    /// # Arguments
    ///
    /// * `current_location` - A `Coord<f64>` representing the current location on the edge.
    /// * `direction_coord` - A `Coord<f64>` representing the target direction along the edge.
    ///
    /// # Returns
    ///
    /// A `f64` representing the distance to the last coordinate in the linestring.
    ///
    fn distance_to_end(&self, current_location: Coord<f64>, direction_coord: Coord<f64>) -> Length;

    /// Calculates a new position on the edge based on the given parameters.
    ///
    /// # Arguments
    ///
    /// * `current_location` - A `Coord<f64>` representing the current location on the edge.
    /// * `distance_to_travel` - A `f64` representing the distance to travel along the edge from the current location.
    /// * `direction_coord` - A `Coord<f64>` representing the target direction along the edge.
    ///
    /// # Returns
    ///
    /// A `Coord<f64>` representing the new position on the edge after traveling the specified distance in the given direction.
    ///
    fn position_on_edge(
        &self,
        current_location: Coord<f64>,
        distance_to_travel: Length,
        direction_coord: Coord<f64>,
    ) -> Coord<f64>;
}

impl RailwayEdgeAlgos for RailwayEdge {
    fn distance_to_end(&self, current_location: Coord<f64>, direction_coord: Coord<f64>) -> Length {
        // Get the points in front of the current_location in the direction of direction_coord
        let points_in_front = points_in_front(&self.path, current_location, direction_coord);

        // If there are no points in front, return 0.0
        if points_in_front.is_empty() {
            return Length::new::<meter>(0.0);
        }

        let mut total_distance = Length::new::<meter>(0.0);
        let mut current_point = current_location;

        for next_point in points_in_front {
            let segment_distance = current_point.distance(&next_point);
            total_distance += segment_distance;

            current_point = next_point;
        }
        total_distance
    }

    fn position_on_edge(
        &self,
        current_location: Coord<f64>,
        distance_to_travel: Length,
        direction_coord: Coord<f64>,
    ) -> Coord<f64> {
        // Get the points in front of the current_location in the direction of direction_coord
        let points_in_front = points_in_front(&self.path, current_location, direction_coord);

        // If there are no points in front, return the current_location
        if points_in_front.is_empty() {
            return current_location;
        }

        // Calculate the remaining distance to travel
        let mut remaining_distance = distance_to_travel.get::<meter>();

        // Iterate through the points in front and find the point where the remaining_distance is reached
        let mut current_point = current_location;
        let mut new_position = current_location;

        for next_point in points_in_front {
            let current_point_geo = Point::new(current_point.x, current_point.y);
            let next_point_geo = Point::new(next_point.x, next_point.y);

            // Use haversine_distance instead of euclidean_distance
            let segment_distance = current_point_geo.haversine_distance(&next_point_geo);

            let ratio = remaining_distance / segment_distance;
            new_position.x = current_point.x + ratio * (next_point.x - current_point.x);
            new_position.y = current_point.y + ratio * (next_point.y - current_point.y);
            if remaining_distance < segment_distance {
                break;
            } else {
                current_point = next_point;
                remaining_distance -= segment_distance;
            }
        }
        new_position
    }
}
