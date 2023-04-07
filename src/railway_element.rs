use crate::Coordinate;
use serde::{ser::Error, Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Debug)]
pub struct RailwayElement {
    pub id: i64,
    pub tags: Option<HashMap<String, String>>,
    pub nodes: Option<Vec<i64>>,
    pub geometry: Option<Vec<Coordinate>>,
}

impl RailwayElement {
    pub fn from_json(json_value: &Value) -> Result<Vec<RailwayElement>, serde_json::Error> {
        let railway_elements = json_value["elements"]
            .as_array()
            .ok_or_else(|| serde_json::Error::custom("Elements parsing error"))?
            .iter()
            .filter_map(|elem| serde_json::from_value::<RailwayElement>(elem.clone()).ok())
            .collect::<Vec<RailwayElement>>();
        Ok(railway_elements)
    }
}
