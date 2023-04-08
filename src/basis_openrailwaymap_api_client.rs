use crate::openrailwaymap_api_client::OpenRailwayMapApiClient;
use anyhow::Result;
use async_trait::async_trait;
use reqwest::Client;
use serde_json::Value;

pub struct BasicOpenRailwayMapApiClient {
    url: Option<String>,
}

#[cfg(target_arch = "wasm32")]
unsafe impl Send for BasicOpenRailwayMapApiClient {}

impl BasicOpenRailwayMapApiClient {
    pub fn new() -> Self {
        BasicOpenRailwayMapApiClient { url: None }
    }

    async fn fetch_by_query(&self, query: &str) -> Result<Value> {
        let client = Client::new();
        let form_data = [("data", query)];

        let response: Value = client
            .post(
                self.url
                    .as_deref()
                    .unwrap_or("https://overpass-api.de/api/interpreter"),
            )
            .form(&form_data)
            .send()
            .await?
            .json()
            .await?;

        Ok(response)
    }
}

impl Default for BasicOpenRailwayMapApiClient {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl OpenRailwayMapApiClient for BasicOpenRailwayMapApiClient {
    async fn connect(&mut self, url: &str) -> Result<()> {
        self.url = Some(url.to_string());

        let client = Client::new();
        client.get(url).send().await?;
        Ok(())
    }

    async fn fetch_by_area_name(&self, area_name: &str) -> Result<Value> {
        let query = format!(
            r#"[out:json];area[name="{}"]->.searchArea;(way(area.searchArea)["railway"="rail"];node(area.searchArea)["railway"="switch"];);out geom;"#,
            area_name
        );

        let response: Value = self.fetch_by_query(&query).await?;
        Ok(response)
    }

    async fn fetch_by_bbox(&self, bbox: &str) -> Result<Value> {
        let query = format!(
            r#"[out:json];(way({})["railway"="rail"];node({})["railway"="switch"];);out geom;"#,
            bbox, bbox
        );

        let response: Value = self.fetch_by_query(&query).await?;
        Ok(response)
    }
}
