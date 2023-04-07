use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Coordinate {
    pub lat: f64,
    pub lon: f64,
}
