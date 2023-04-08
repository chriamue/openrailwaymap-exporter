#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

#[cfg(target_arch = "wasm32")]
mod app;

mod basis_openrailwaymap_api_client;
mod coordinate;
mod export;
mod openrailwaymap_api_client;
mod railway_element;
mod railway_model;
mod railway_processing;

#[cfg(target_arch = "wasm32")]
pub use self::app::*;
pub use self::basis_openrailwaymap_api_client::BasicOpenRailwayMapApiClient;
pub use self::coordinate::Coordinate;
pub use self::export::*;
pub use self::openrailwaymap_api_client::OpenRailwayMapApiClient;
pub use self::railway_element::{ElementType, RailwayElement};
pub use self::railway_model::*;
pub use self::railway_processing::*;
