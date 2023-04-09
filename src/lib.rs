#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

#[cfg(target_arch = "wasm32")]
mod app;

mod export;
mod railway_api_client;
mod railway_model;

#[cfg(test)]
pub mod tests;

/// This prelude re-exports the most commonly used items from the library.
pub mod prelude {
    #[cfg(target_arch = "wasm32")]
    pub use super::app::*;
    pub use super::export::*;
    pub use super::railway_api_client::overpass_api_client;
    pub use super::railway_api_client::{OverpassApiClient, RailwayApiClient};
    pub use super::railway_model::*;
}
