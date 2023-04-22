use bevy::prelude::{Resource, Vec3};
use geo::Coord;

/// The `Projection` resource is responsible for converting geographical coordinates
/// (latitude and longitude) to view coordinates, taking into account the view dimensions
/// and the bounding box of a given `RailwayGraph`.
#[derive(Default, Resource)]
pub struct Projection {
    view_width: f64,
    view_height: f64,
    bounding_box: Option<(Coord, Coord)>,
}

impl Projection {
    /// Creates a new `Projection` resource with the specified view dimensions.
    ///
    /// # Arguments
    ///
    /// * `view_width` - The width of the view in pixels.
    /// * `view_height` - The height of the view in pixels.
    ///
    pub fn new(view_width: f64, view_height: f64) -> Self {
        Self {
            view_width,
            view_height,
            bounding_box: None,
        }
    }

    /// Sets the bounding box of the `Projection` resource based on the minimum and maximum coordinates.
    ///
    /// # Arguments
    ///
    /// * `min_coord` - The minimum coordinate of the bounding box.
    /// * `max_coord` - The maximum coordinate of the bounding box.
    ///
    pub fn set_bounding_box(&mut self, min_coord: Coord, max_coord: Coord) {
        self.bounding_box = Some((min_coord, max_coord));
    }

    /// Projects the given geographical coordinate to a view coordinate.
    ///
    /// # Arguments
    ///
    /// * `coord` - A geographical coordinate (latitude and longitude).
    ///
    /// # Returns
    ///
    /// * `Option<Vec3>` - A view coordinate as a 3D vector if the bounding box is set, or `None` otherwise.
    ///
    pub fn project(&self, coord: Coord) -> Option<Vec3> {
        if let Some((min_coord, max_coord)) = self.bounding_box {
            let x = ((coord.x - min_coord.x) / (max_coord.x - min_coord.x)) * self.view_width;
            let y = ((coord.y - min_coord.y) / (max_coord.y - min_coord.y)) * self.view_height;

            // Center the projection
            let centered_x = x - self.view_width / 2.0;
            let centered_y = y - self.view_height / 2.0;

            Some(Vec3::new(centered_x as f32, centered_y as f32, 0.0))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_projection() {
        let mut projection = Projection::new(1000.0, 1000.0);
        projection.set_bounding_box(Coord { x: 0.0, y: 0.0 }, Coord { x: 10.0, y: 10.0 });

        let coord = Coord { x: 0.0, y: 0.0 };
        let projected_coord = projection.project(coord).unwrap();

        assert_eq!(projected_coord, Vec3::new(-500.0, -500.0, 0.0));
        let coord = Coord { x: 5.0, y: 5.0 };
        let projected_coord = projection.project(coord).unwrap();

        assert_eq!(projected_coord, Vec3::new(0.0, 0.0, 0.0));
    }
}
