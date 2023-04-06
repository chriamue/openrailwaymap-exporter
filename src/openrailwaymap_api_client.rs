use std::error::Error;

use crate::railway_element::RailwayElement;
use async_trait::async_trait;

#[async_trait]
pub trait OpenRailwayMapApiClient {
    async fn connect(&mut self, url: &str) -> Result<(), Box<dyn Error>>;
    async fn fetch_railway_elements_by_area_name(
        &self,
        area_name: &str,
    ) -> Result<Vec<RailwayElement>, Box<dyn Error>>;
    async fn fetch_railway_elements_by_bbox(
        &self,
        bbox: &str,
    ) -> Result<Vec<RailwayElement>, Box<dyn Error>>;
}
