//! Types used throughout the project.
//!
/// A unique identifier for a node in a `RailwayGraph`.
pub type NodeId = i64;

/// A unique identifier for an edge in a `RailwayGraph`.
pub type EdgeId = i64;

/// A unique identifier for a railway object.
pub type RailwayObjectId = i64;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_id_creation_and_assignment() {
        let node_id: NodeId = 42;
        assert_eq!(node_id, 42);
    }

    #[test]
    fn test_edge_id_creation_and_assignment() {
        let edge_id: EdgeId = 24;
        assert_eq!(edge_id, 24);
    }

    #[test]
    fn test_railway_object_id_creation_and_assignment() {
        let railway_object_id: RailwayObjectId = 10;
        assert_eq!(railway_object_id, 10);
    }
}
