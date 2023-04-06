use crate::openrailwaymap_api_client::OpenRailwayMapApiClient;
use crate::railway_element::RailwayElement;
use async_trait::async_trait;
use reqwest::Client;
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
}

impl BasicOpenRailwayMapApiClient {
    pub fn new() -> Self {
        let client = Client::new();
        BasicOpenRailwayMapApiClient { client }
    }
}

#[async_trait]
impl OpenRailwayMapApiClient for BasicOpenRailwayMapApiClient {
    async fn connect(&self, bbox: &str) -> Result<Vec<RailwayElement>, Box<dyn Error>> {
        let query = format!(r#"[out:json];way({})["railway"="rail"];out geom;"#, bbox);
        let form_data = [("data", query)];

        let response = self
            .client
            .post("https://overpass-api.de/api/interpreter")
            .form(&form_data)
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;

        let elements = response["elements"].as_array().ok_or_else(|| ApiError {
            message: "Invalid API response".to_string(),
        })?;

        let railway_elements: Vec<RailwayElement> = elements
            .iter()
            .filter_map(|e| serde_json::from_value(e.clone()).ok())
            .collect();

        Ok(railway_elements)
    }
}
