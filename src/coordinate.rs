use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Coordinate {
    pub lat: f64,
    pub lon: f64,
}
