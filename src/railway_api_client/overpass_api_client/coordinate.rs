use serde::{Deserialize, Serialize};

/// A `Coordinate` represents a geographic coordinate with latitude and longitude.
///
/// The `Coordinate` struct has two fields, `lat` for latitude and `lon` for longitude. This struct
/// can be used for storing and manipulating geographic coordinates.
///
/// # Example
///
/// ```
/// use openrailwaymap_exporter::prelude::overpass_api_client::Coordinate;
///
/// let coord = Coordinate { lat: 50.1109, lon: 8.6821 };
/// println!("Latitude: {}, Longitude: {}", coord.lat, coord.lon);
/// ```
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Coordinate {
    /// The latitude coordinate, in decimal degrees.
    pub lat: f64,
    /// The longitude coordinate, in decimal degrees.
    pub lon: f64,
}

#[cfg(test)]
mod tests {
    use super::Coordinate;

    #[test]
    fn test_coordinate() {
        let coord = Coordinate {
            lat: 50.1109,
            lon: 8.6821,
        };

        assert_eq!(coord.lat, 50.1109);
        assert_eq!(coord.lon, 8.6821);
    }
}
