use geo::{coord, ClosestPoint, Coord, EuclideanDistance, LineString, Point};

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
