use anyhow::Result;
use async_trait::async_trait;
use serde_json::Value;

/// overpass api client
pub mod overpass_api_client;
pub use overpass_api_client::OverpassApiClient;

/// A trait for implementing an Railway API client.
///
/// `RailwayApiClient` is an asynchronous trait that provides a common interface
/// for fetching data by area name or bounding box.
///
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait RailwayApiClient {
    /// Connect to the OpenRailwayMap API using the specified URL.
    ///
    /// # Arguments
    ///
    /// * `url` - The URL of the OpenRailwayMap API.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or failure.
    async fn connect(&mut self, url: &str) -> Result<()>;

    /// Fetch OpenRailwayMap data by area name.
    ///
    /// # Arguments
    ///
    /// * `area_name` - The name of the area for which to fetch data.
    ///
    /// # Returns
    ///
    /// A `Result` containing a JSON `Value` with the fetched data on success, or an error on failure.
    async fn fetch_by_area_name(&self, area_name: &str) -> Result<Value>;

    /// Fetch OpenRailwayMap data by bounding box.
    ///
    /// # Arguments
    ///
    /// * `bbox` - A string representing the bounding box for which to fetch data.
    ///
    /// # Returns
    ///
    /// A `Result` containing a JSON `Value` with the fetched data on success, or an error on failure.
    async fn fetch_by_bbox(&self, bbox: &str) -> Result<Value>;
}
