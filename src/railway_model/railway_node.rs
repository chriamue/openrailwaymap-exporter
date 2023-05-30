use geo::Coord;
use transit_grid::prelude::TransitNode;

/// A railway node.
pub type RailwayNode = TransitNode<Coord<f64>>;

#[cfg(test)]
mod tests {
    use super::RailwayNode;
    use geo::coord;

    #[test]
    fn test_railway_node_creation() {
        let node = RailwayNode {
            id: 1,
            location: coord! { x: 8.6090232, y: 50.1191127 },
        };

        assert_eq!(node.id, 1);
        assert_eq!(node.location.y, 50.1191127);
        assert_eq!(node.location.x, 8.6090232);
    }

    #[test]
    fn test_railway_node_clone() {
        let node = RailwayNode {
            id: 1,
            location: coord! { x: 8.6090232, y: 50.1191127 },
        };

        let cloned_node = node.clone();
        assert_eq!(node, cloned_node);
    }
}
