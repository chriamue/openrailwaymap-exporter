//! A module containing algorithms for working with geographical data structures.
//!
use geo::{coord, Coord, EuclideanDistance, LineString, Point};

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
    let position_point = Point::new(position.x, position.y);

    let mut min_distance = f64::MAX;
    let mut closest_position = position;

    for p in linestring.points() {
        let distance = p.euclidean_distance(&position_point);

        if distance < min_distance {
            min_distance = distance;
            closest_position = coord! { x: p.x(), y: p.y() };
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
    let closest_point = Point::new(
        closest_point_in_linestring(current_location, linestring).x,
        closest_point_in_linestring(current_location, linestring).y,
    );
    let current_location_point = Point::new(current_location.x, current_location.y);
    let target_direction_point = Point::new(target_direction.x, target_direction.y);

    let target_vector = target_direction_point - current_location_point;
    let mut points_in_front = Vec::new();

    for p in linestring.points() {
        let point_vector = p - closest_point;

        if target_vector.dot(point_vector) > 0.0 {
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
    let (x1, y1) = (start_coord.x, start_coord.y);
    let (x2, y2) = (middle_coord.x, middle_coord.y);
    let (x3, y3) = (end_coord.x, end_coord.y);

    let between_x = (x2 >= x1 && x2 <= x3) || (x2 >= x3 && x2 <= x1);
    let between_y = (y2 >= y1 && y2 <= y3) || (y2 >= y3 && y2 <= y1);

    between_x && between_y
}

#[cfg(test)]
mod tests {
    use super::*;
    use geo::{coord, line_string};

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
        }
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

        let points_in_front = points_in_front(&linestring, current_location, target_direction);

        assert_eq!(
            points_in_front,
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
            end_coord
        ));

        let not_middle_coord = coord! { x: 40.0, y: 40.0 };

        assert!(!is_middle_coord_between(
            start_coord,
            not_middle_coord,
            end_coord
        ));
    }
}
