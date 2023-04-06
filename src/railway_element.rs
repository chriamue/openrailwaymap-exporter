use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct RailwayElement {
    pub id: i64,
    pub tags: Option<std::collections::HashMap<String, String>>,
    pub nodes: Option<Vec<i64>>,
}
