/// Represents a railway edge with a unique ID and a length.
///
/// # Examples
///
/// ```
/// use openrailwaymap_exporter::RailwayEdge;
///
/// let edge = RailwayEdge {
///     id: 1,
///     length: 1500.0,
/// };
/// assert_eq!(edge.id, 1);
/// ```
#[derive(Debug, PartialEq)]
pub struct RailwayEdge {
    /// The ID of the edge, typically corresponding to the ID of the underlying `RailwayElement` (e.g., way).
    pub id: i64,
    /// The length of the railway segment in meters.
    pub length: f64,
}

#[cfg(test)]
mod tests {
    use super::RailwayEdge;

    #[test]
    fn test_railway_edge_creation() {
        let edge = RailwayEdge {
            id: 1,
            length: 1500.0,
        };

        assert_eq!(edge.id, 1);
        assert_eq!(edge.length, 1500.0);
    }

    #[test]
    fn test_railway_edge_comparison() {
        let edge1 = RailwayEdge {
            id: 1,
            length: 1500.0,
        };

        let edge2 = RailwayEdge {
            id: 1,
            length: 1500.0,
        };

        let edge3 = RailwayEdge {
            id: 2,
            length: 2500.0,
        };

        assert_eq!(edge1, edge2);
        assert_ne!(edge1, edge3);
    }
}
