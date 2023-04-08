/// Represents a railway node with a unique ID and geographic coordinates (latitude and longitude).
///
/// # Examples
///
/// ```
/// use openrailwaymap_exporter::RailwayNode;
///
/// let node = RailwayNode {
///     id: 1,
///     lat: 50.1191127,
///     lon: 8.6090232,
/// };
/// assert_eq!(node.id, 1);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct RailwayNode {
    /// The unique identifier of the railway node.
    pub id: i64,
    /// The latitude coordinate of the railway node.
    pub lat: f64,
    /// The longitude coordinate of the railway node.
    pub lon: f64,
}

#[cfg(test)]
mod tests {
    use super::RailwayNode;

    #[test]
    fn test_railway_node_creation() {
        let node = RailwayNode {
            id: 1,
            lat: 50.1191127,
            lon: 8.6090232,
        };

        assert_eq!(node.id, 1);
        assert_eq!(node.lat, 50.1191127);
        assert_eq!(node.lon, 8.6090232);
    }

    #[test]
    fn test_railway_node_clone() {
        let node = RailwayNode {
            id: 1,
            lat: 50.1191127,
            lon: 8.6090232,
        };

        let cloned_node = node.clone();
        assert_eq!(node, cloned_node);
    }
}
