use std::error::Error;

use crate::railway_element::RailwayElement;
use async_trait::async_trait;

#[async_trait]
pub trait OpenRailwayMapApiClient {
    async fn connect(&self, bbox: &str) -> Result<Vec<RailwayElement>, Box<dyn Error>>;
}
