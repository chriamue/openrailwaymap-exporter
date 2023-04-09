use crate::Coordinate;

/// Represents a railway edge with a unique ID, a length, and a path.
///
/// # Examples
///
/// ```
/// use openrailwaymap_exporter::{Coordinate, RailwayEdge};
///
/// let edge = RailwayEdge {
///     id: 1,
///     length: 1500.0,
///     path: vec![
///         Coordinate { lat: 50.1109, lon: 8.6821 },
///         Coordinate { lat: 50.1209, lon: 8.6921 },
///     ],
/// };
/// assert_eq!(edge.id, 1);
/// ```
#[derive(Debug, PartialEq)]
pub struct RailwayEdge {
    /// The ID of the edge, typically corresponding to the ID of the underlying `RailwayElement` (e.g., way).
    pub id: i64,
    /// The length of the railway segment in meters.
    pub length: f64,
    /// The path of the edge, stored as a vector of `Coordinate` structs.
    pub path: Vec<Coordinate>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_railway_edge_creation() {
        let edge = RailwayEdge {
            id: 1,
            length: 1500.0,
            path: vec![
                Coordinate {
                    lat: 50.1109,
                    lon: 8.6821,
                },
                Coordinate {
                    lat: 50.1209,
                    lon: 8.6921,
                },
            ],
        };

        assert_eq!(edge.id, 1);
        assert_eq!(edge.length, 1500.0);
    }

    #[test]
    fn test_railway_edge_comparison() {
        let edge1 = RailwayEdge {
            id: 1,
            length: 1500.0,
            path: vec![
                Coordinate {
                    lat: 50.1109,
                    lon: 8.6821,
                },
                Coordinate {
                    lat: 50.1209,
                    lon: 8.6921,
                },
            ],
        };

        let edge2 = RailwayEdge {
            id: 1,
            length: 1500.0,
            path: vec![
                Coordinate {
                    lat: 50.1109,
                    lon: 8.6821,
                },
                Coordinate {
                    lat: 50.1209,
                    lon: 8.6921,
                },
            ],
        };

        let edge3 = RailwayEdge {
            id: 2,
            length: 2500.0,
            path: vec![
                Coordinate {
                    lat: 50.1209,
                    lon: 8.6921,
                },
                Coordinate {
                    lat: 50.1309,
                    lon: 8.7021,
                },
            ],
        };

        assert_eq!(edge1, edge2);
        assert_ne!(edge1, edge3);
    }
}
