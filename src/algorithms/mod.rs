//! A module containing algorithms for working with geographical data structures.
//!
use geo::{coord, Coord, LineString, Point};
use uom::si::length::meter;

mod distance;
pub use distance::Distance;

/// Returns the closest point of points in a given `LineString` to a given `Coord`.
///
/// # Examples
///
/// ```
/// use geo::{line_string, coord};
/// use openrailwaymap_exporter::algorithms::closest_point_in_linestring;
///
/// let linestring = line_string![
///     coord! { x: 0.0, y: 0.0 },
///     coord! { x: 0.0, y: 10.0 },
///     coord! { x: 50.0, y: 50.0 },
///     coord! { x: 100.0, y: 100.0 },
/// ];
/// let position = coord! { x: 10.0, y: 20.0 };
/// let closest_position = closest_point_in_linestring(position, &linestring);
///
/// assert_eq!(closest_position, coord! { x: 0.0, y: 10.0 });
/// ```
pub fn closest_point_in_linestring(
    position: Coord<f64>,
    linestring: &LineString<f64>,
) -> Coord<f64> {
    let mut min_distance = f64::MAX;
    let mut closest_position = position;

    for p in linestring.points() {
        let p = coord! { x: p.x(), y: p.y() };
        let distance = p.distance(&position);

        if distance.get::<meter>() < min_distance {
            min_distance = distance.get::<meter>();
            closest_position = p;
        }
    }

    closest_position
}

/// Returns the points in front of the current location in a given `LineString`.
///
/// # Examples
///
/// ```
/// use geo::{line_string, coord};
/// use openrailwaymap_exporter::algorithms::{closest_point_in_linestring, points_in_front};
///
/// let linestring = line_string![
///     coord! { x: 0.0, y: 0.0 },
///     coord! { x: 0.0, y: 10.0 },
///     coord! { x: 50.0, y: 50.0 },
///     coord! { x: 100.0, y: 100.0 },
/// ];
/// let current_location = coord! { x: 10.0, y: 20.0 };
/// let target_direction = coord! { x: 100.0, y: 100.0 };
///
/// let points = points_in_front(&linestring, current_location, target_direction);
///
/// assert_eq!(points, vec![coord! { x: 50.0, y: 50.0 }, coord! { x: 100.0, y: 100.0 }]);
/// ```
pub fn points_in_front(
    linestring: &LineString<f64>,
    current_location: Coord<f64>,
    target_direction: Coord<f64>,
) -> Vec<Coord<f64>> {
    let current_location_point = Point::new(current_location.x, current_location.y);
    let target_direction_point = Point::new(target_direction.x, target_direction.y);

    let first_point = linestring.into_iter().next().unwrap();
    let last_point = linestring.into_iter().last().unwrap();

    let distance_to_first_point = current_location.distance(first_point);
    let distance_to_last_point = target_direction.distance(last_point);

    let linestring = if distance_to_first_point < distance_to_last_point {
        LineString::from(
            linestring
                .into_iter()
                .rev()
                .cloned()
                .collect::<Vec<Coord>>(),
        )
    } else {
        linestring.clone()
    };

    let target_vector = target_direction_point - current_location_point;
    let mut points_in_front = Vec::new();

    for p in linestring.points() {
        let point_vector = p - current_location_point;

        if target_vector.dot(point_vector) >= 0.0 {
            points_in_front.push(coord! { x: p.x(), y: p.y() });
        }
    }

    points_in_front
}

/// Determines if the middle coordinate is between the start and end coordinates along both x and y axes.
///
/// This function assumes the three coordinates are collinear.
///
/// # Arguments
///
/// * `start_coord` - A `Coord<f64>` representing the start coordinate.
/// * `middle_coord` - A `Coord<f64>` representing the middle coordinate.
/// * `end_coord` - A `Coord<f64>` representing the end coordinate.
///
/// # Returns
///
/// A `bool` indicating whether `middle_coord` is between `start_coord` and `end_coord`.
///
pub fn is_middle_coord_between(
    start_coord: Coord<f64>,
    middle_coord: Coord<f64>,
    end_coord: Coord<f64>,
) -> bool {
    let distance_start_end = start_coord.distance(&end_coord);
    let distance_start_middle = start_coord.distance(&middle_coord);
    let distance_middle_end = middle_coord.distance(&end_coord);

    distance_start_end > distance_start_middle && distance_start_end > distance_middle_end
}

#[cfg(test)]
mod tests {
    use super::*;
    use geo::{coord, line_string};

    #[test]
    fn test_closest_point_in_linestring() {
        let linestring = line_string![
            coord! { x: 0.0, y: 0.0 },
            coord! { x: 0.0, y: 10.0 },
            coord! { x: 50.0, y: 50.0 },
            coord! { x: 100.0, y: 100.0 },
        ];
        let position = coord! { x: 10.0, y: 20.0 };
        let closest_position = closest_point_in_linestring(position, &linestring);

        assert_eq!(closest_position, coord! { x: 0.0, y: 10.0 });

        let linestring = line_string![
            coord! { x: 0.0, y: 0.0 },
            coord! { x: 10.0, y: 10.0 },
            coord! { x: 20.0, y: 20.0 },
            coord! { x: 30.0, y: 30.0 },
        ];
        let current_location = coord! { x: 5.0, y: 5.0 };
        let closest_position = closest_point_in_linestring(current_location, &linestring);

        assert_eq!(closest_position, coord! { x: 10.0, y: 10.0 });
    }

    #[test]
    fn test_points_in_front() {
        let linestring = line_string![
            coord! { x: 0.0, y: 0.0 },
            coord! { x: 10.0, y: 10.0 },
            coord! { x: 20.0, y: 20.0 },
            coord! { x: 30.0, y: 30.0 },
        ];
        let current_location = coord! { x: 5.0, y: 5.0 };
        let target_direction = coord! { x: 25.0, y: 25.0 };

        let calculated_points_in_front =
            points_in_front(&linestring, current_location, target_direction);

        assert_eq!(
            calculated_points_in_front,
            vec![
                coord! { x: 10.0, y: 10.0 },
                coord! { x: 20.0, y: 20.0 },
                coord! { x: 30.0, y: 30.0 },
            ]
        );
    }

    #[test]
    fn test_is_middle_coord_between() {
        let start_coord = coord! { x: 10.0, y: 10.0 };
        let middle_coord = coord! { x: 20.0, y: 20.0 };
        let end_coord = coord! { x: 30.0, y: 30.0 };

        assert!(is_middle_coord_between(
            start_coord,
            middle_coord,
            end_coord,
        ));

        let not_middle_coord = coord! { x: 40.0, y: 40.0 };

        assert!(!is_middle_coord_between(
            start_coord,
            not_middle_coord,
            end_coord,
        ));
    }
}
