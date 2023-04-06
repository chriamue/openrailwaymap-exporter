use crate::openrailwaymap_api_client::OpenRailwayMapApiClient;
use crate::railway_element::RailwayElement;
use async_trait::async_trait;
use reqwest::Client;
use serde_json::Value;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct ApiError {
    pub message: String,
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for ApiError {}

pub struct BasicOpenRailwayMapApiClient {
    client: Client,
    url: Option<String>,
}

impl BasicOpenRailwayMapApiClient {
    pub fn new() -> Self {
        let client = Client::new();
        BasicOpenRailwayMapApiClient { client, url: None }
    }

    async fn fetch_railway_elements_by_query(
        &self,
        query: &str,
    ) -> Result<Vec<RailwayElement>, Box<dyn Error>> {
        let form_data = [("data", query)];

        let response: Value = self
            .client
            .post("https://overpass-api.de/api/interpreter")
            .form(&form_data)
            .send()
            .await?
            .json()
            .await?;

        let railway_elements = response["elements"]
            .as_array()
            .ok_or_else(|| ApiError {
                message: "Invalid API response".to_string(),
            })?
            .iter()
            .filter_map(|elem| serde_json::from_value::<RailwayElement>(elem.clone()).ok())
            .collect::<Vec<RailwayElement>>();

        Ok(railway_elements)
    }
}

#[async_trait]
impl OpenRailwayMapApiClient for BasicOpenRailwayMapApiClient {
    async fn connect(&mut self, url: &str) -> Result<(), Box<dyn Error>> {
        self.url = Some(url.to_string());
        self.client.get(url).send().await?;
        Ok(())
    }

    async fn fetch_railway_elements_by_area_name(
        &self,
        area_name: &str,
    ) -> Result<Vec<RailwayElement>, Box<dyn Error>> {
        let query = format!(
            r#"[out:json];area[name="{}"]->.searchArea;(way(area.searchArea)["railway"="rail"];way(area.searchArea)["railway"="switch"];);out geom;"#,
            area_name
        );

        self.fetch_railway_elements_by_query(&query).await
    }

    async fn fetch_railway_elements_by_bbox(
        &self,
        bbox: &str,
    ) -> Result<Vec<RailwayElement>, Box<dyn Error>> {
        let query = format!(
            r#"[out:json];(way({})["railway"="rail"];way({})["railway"="switch"];);out geom;"#,
            bbox, bbox
        );

        self.fetch_railway_elements_by_query(&query).await
    }
}
