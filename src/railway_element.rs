use serde::Deserialize;
use std::collections::HashMap;

use crate::Coordinate;

#[derive(Deserialize, Debug)]
pub struct RailwayElement {
    pub id: i64,
    pub tags: Option<HashMap<String, String>>,
    pub nodes: Option<Vec<i64>>,
    pub geometry: Option<Vec<Coordinate>>,
}
