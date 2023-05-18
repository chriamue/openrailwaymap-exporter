use super::RailwayApiClient;
use anyhow::Result;
use async_trait::async_trait;
use reqwest::Client;
use serde_json::Value;

/// A basic client for the OpenRailwayMap API.
///
pub struct OverpassApiClient {
    url: Option<String>,
}

#[cfg(target_arch = "wasm32")]
unsafe impl Send for OverpassApiClient {}

impl OverpassApiClient {
    /// Creates a new `OverpassApiClient` with no specified API URL.
    pub fn new() -> Self {
        OverpassApiClient { url: None }
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

impl Default for OverpassApiClient {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl RailwayApiClient for OverpassApiClient {
    async fn connect(&mut self, url: &str) -> Result<()> {
        self.url = Some(url.to_string());

        let client = Client::new();
        client.get(url).send().await?;
        Ok(())
    }

    async fn fetch_by_area_name(&self, area_name: &str) -> Result<Value> {
        let query = format!(
            r#"[out:json];area[name="{}"]->.searchArea;(way(area.searchArea)["railway"="rail"];node(area.searchArea)["railway"="switch"];node(area.searchArea)["railway"="buffer_stop"];node(area.searchArea)["railway"="railway_crossing"];);out geom;"#,
            area_name
        );

        let response: Value = self.fetch_by_query(&query).await?;
        Ok(response)
    }

    async fn fetch_by_bbox(&self, bbox: &str) -> Result<Value> {
        let query = format!(
            r#"[out:json];(way({})["railway"="rail"];node({})["railway"="switch"];node({})["railway"="buffer_stop"];node({})["railway"="railway_crossing"];);out geom;"#,
            bbox, bbox, bbox, bbox
        );

        let response: Value = self.fetch_by_query(&query).await?;
        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_json_vilbel;
    use mockito::{mock, server_url};

    #[cfg_attr(not(target_arch = "wasm32"), tokio::test)]
    #[cfg_attr(target_arch = "wasm32", ignore)]
    async fn test_connect() {
        let mock = mock("GET", "/").with_status(200).create();

        let mut client = OverpassApiClient::new();
        let result = client.connect(&server_url()).await;

        mock.assert();
        assert!(result.is_ok());
    }

    #[cfg_attr(not(target_arch = "wasm32"), tokio::test)]
    #[cfg_attr(target_arch = "wasm32", ignore)]
    async fn test_fetch_by_area_name() {
        let test_json = test_json_vilbel();
        let query = r#"[out:json];area[name="Bad Vilbel"]->.searchArea;(way(area.searchArea)["railway"="rail"];node(area.searchArea)["railway"="switch"];node(area.searchArea)["railway"="buffer_stop"];node(area.searchArea)["railway"="railway_crossing"];);out geom;"#;
        let mock = mock("POST", "/api/interpreter")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(&serde_json::to_string(&test_json).unwrap())
            .match_header("content-type", "application/x-www-form-urlencoded")
            .match_body(mockito::Matcher::UrlEncoded(
                "data".to_string(),
                query.to_string(),
            ))
            .create();

        let mut client = OverpassApiClient::new();
        client
            .connect(&format!("{}/api/interpreter", server_url()))
            .await
            .unwrap();
        let result = client.fetch_by_area_name("Bad Vilbel").await;

        mock.assert();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), test_json);
    }


    #[cfg_attr(not(target_arch = "wasm32"), tokio::test)]
    #[cfg_attr(target_arch = "wasm32", ignore)]
    async fn test_fetch_by_bbox() {
        let test_json = test_json_vilbel();
        let bbox = "1,2,3,4";
        let query = format!(
            r#"[out:json];(way({})["railway"="rail"];node({})["railway"="switch"];node({})["railway"="buffer_stop"];node({})["railway"="railway_crossing"];);out geom;"#,
            bbox, bbox, bbox, bbox
        );
        let mock = mock("POST", "/api/interpreter")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(&serde_json::to_string(&test_json).unwrap())
            .match_header("content-type", "application/x-www-form-urlencoded")
            .match_body(mockito::Matcher::UrlEncoded(
                "data".to_string(),
                query.to_string(),
            ))
            .create();

        let mut client = OverpassApiClient::new();
        client
            .connect(&format!("{}/api/interpreter", server_url()))
            .await
            .unwrap();
        let result = client.fetch_by_bbox(bbox).await;

        mock.assert();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), test_json);
    }
}
