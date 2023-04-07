use anyhow::Result;
use async_trait::async_trait;
use serde_json::Value;

#[async_trait]
pub trait OpenRailwayMapApiClient {
    async fn connect(&mut self, url: &str) -> Result<()>;
    async fn fetch_by_area_name(&self, area_name: &str) -> Result<Value>;
    async fn fetch_by_bbox(&self, bbox: &str) -> Result<Value>;
}
