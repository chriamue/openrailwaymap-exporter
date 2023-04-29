//! A module to handle distance calculations for different types of points using Haversine
//! and Euclidean distance calculations.
use geo::{Coord, EuclideanDistance, HaversineDistance, Point};
use uom::si::f64::Length;
use uom::si::length::meter;

/// A trait that defines a method for calculating the distance between two points
/// of the same type. Implementations are provided for `Coord<f64>` and `Point<f64>`.
pub trait Distance {
    /// Calculate the distance between `self` and `other` and return the result
    /// as a `Length` in meters.
    fn distance(&self, other: &Self) -> Length;
}

/// Implementation of `Distance` for `Coord<f64>`. The distance is calculated
/// using the Haversine formula.
impl Distance for Coord<f64> {
    fn distance(&self, other: &Coord<f64>) -> Length {
        let point1 = Point::new(self.x, self.y);
        let point2 = Point::new(other.x, other.y);
        let haversine_distance = point1.haversine_distance(&point2);
        Length::new::<meter>(haversine_distance)
    }
}

/// Implementation of `Distance` for `Point<f64>`. The distance is calculated
/// using the Euclidean formula.
impl Distance for Point<f64> {
    fn distance(&self, other: &Point<f64>) -> Length {
        let euclidean_distance = self.euclidean_distance(other);
        Length::new::<meter>(euclidean_distance)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use geo::coord;

    #[test]
    fn coord_haversine_distance() {
        let coord1 = coord! { x:21.0122287, y: 52.2296756 };
        let coord2 = coord! {x: 16.9251681, y:52.406374 };
        let distance = coord1.distance(&coord2);

        assert!((distance.get::<meter>() - 278_458.0).abs() < 1.0);
    }

    #[test]
    fn point_euclidean_distance() {
        let point1 = Point::new(1.0, 1.0);
        let point2 = Point::new(4.0, 5.0);
        let distance = point1.distance(&point2);

        assert_eq!(distance.get::<meter>(), 5.0);
    }
}
