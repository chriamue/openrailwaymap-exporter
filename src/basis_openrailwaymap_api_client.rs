use crate::openrailwaymap_api_client::OpenRailwayMapApiClient;
use anyhow::Result;
use async_trait::async_trait;
use reqwest::Client;
use serde_json::Value;

pub struct BasicOpenRailwayMapApiClient {
    client: Client,
    url: Option<String>,
}

impl BasicOpenRailwayMapApiClient {
    pub fn new() -> Self {
        let client = Client::new();
        BasicOpenRailwayMapApiClient { client, url: None }
    }

    async fn fetch_by_query(&self, query: &str) -> Result<Value> {
        let form_data = [("data", query)];

        let response: Value = self
            .client
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

#[async_trait]
impl OpenRailwayMapApiClient for BasicOpenRailwayMapApiClient {
    async fn connect(&mut self, url: &str) -> Result<()> {
        self.url = Some(url.to_string());
        self.client.get(url).send().await?;
        Ok(())
    }

    async fn fetch_by_area_name(&self, area_name: &str) -> Result<Value> {
        let query = format!(
            r#"[out:json];area[name="{}"]->.searchArea;(way(area.searchArea)["railway"="rail"];node(area.searchArea)["railway"="switch"];);out geom;"#,
            area_name
        );

        let response: Value = self.fetch_by_query(&query).await?.into();
        Ok(response)
    }

    async fn fetch_by_bbox(&self, bbox: &str) -> Result<Value> {
        let query = format!(
            r#"[out:json];(way({})["railway"="rail"];node({})["railway"="switch"];);out geom;"#,
            bbox, bbox
        );

        let response: Value = self.fetch_by_query(&query).await?.into();
        Ok(response)
    }
}
