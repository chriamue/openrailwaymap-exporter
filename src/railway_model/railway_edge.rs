use geo_types::{Coord, LineString};

use crate::types::NodeId;

/// Represents a railway edge with a unique ID, a length, and a path.
///
/// # Examples
///
/// ```
/// use openrailwaymap_exporter::prelude::RailwayEdge;
/// use geo_types::{coord, LineString};
///
/// let edge = RailwayEdge {
///     id: 1,
///     length: 1500.0,
///     path: LineString::from(vec![
///         coord! { x: 8.6821, y: 50.1109 },
///         coord! { x: 8.6921, y: 50.1209 },
///     ]),
///     source: 2,
///     target: 3
/// };
/// assert_eq!(edge.id, 1);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct RailwayEdge {
    /// The ID of the edge, typically corresponding to the ID of the underlying `RailwayElement` (e.g., way).
    pub id: i64,
    /// The length of the railway segment in meters.
    pub length: f64,
    /// The path of the edge, stored as a `LineString`.
    pub path: LineString<f64>,
    /// Source node identifier
    pub source: NodeId,
    /// Target node identifier
    pub target: NodeId,
}

impl RailwayEdge {
    /// Coordinates of source node
    pub fn source_coordinate(&self) -> Coord<f64> {
        self.path.points().next().unwrap_or_default().0
    }
}

#[cfg(test)]
mod tests {
    use geo_types::coord;

    use super::*;

    #[test]
    fn test_railway_edge_creation() {
        let edge = RailwayEdge {
            id: 1,
            length: 1500.0,
            path: LineString::from(vec![
                coord! { x: 8.6821, y: 50.1109 },
                coord! { x: 8.6921, y: 50.1209 },
            ]),
            source: 0,
            target: 0,
        };

        assert_eq!(edge.id, 1);
        assert_eq!(edge.length, 1500.0);
    }

    #[test]
    fn test_railway_edge_comparison() {
        let edge1 = RailwayEdge {
            id: 1,
            length: 1500.0,
            path: LineString::from(vec![
                coord! { x: 8.6821, y: 50.1109 },
                coord! { x: 8.6921, y: 50.1209 },
            ]),
            source: 0,
            target: 0,
        };

        let edge2 = RailwayEdge {
            id: 1,
            length: 1500.0,
            path: LineString::from(vec![
                coord! { x: 8.6821, y: 50.1109 },
                coord! { x: 8.6921, y: 50.1209 },
            ]),
            source: 0,
            target: 0,
        };

        let edge3 = RailwayEdge {
            id: 2,
            length: 2500.0,
            path: LineString::from(vec![
                coord! { x: 8.6921, y: 50.1209 },
                coord! { x: 8.7021, y: 50.1309 },
            ]),
            source: 0,
            target: 0,
        };

        assert_eq!(edge1, edge2);
        assert_ne!(edge1, edge3);
    }
}
