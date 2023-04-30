//! # OpenRailwayMap Exporter
//!
//! A library to fetch railway infrastructure data from an API, process it, and export the results.
//!
//! The openrailwaymap_exporter crate is designed to help you work with railway infrastructure data.
//! It provides an API client for fetching data, a railway model for representing and processing the
//! data, and an export module for generating various output formats.
//!
//! The primary modules included in the crate are:
//! - railway_api_client: Contains the API client to fetch railway infrastructure data.
//! - railway_model: Contains data structures and functions to work with the railway infrastructure data.
//! - export: Provides functionality to export the railway data in different formats.
//! - simulation: Handles the simulation components, including agent decisions, environment, and execution.
//! - ai: Contains modules for the AI components, including reinforcement learning train agents and their state representation.
//! - app: Provides a web application for displaying and interacting with the data (only available when targeting WebAssembly).
#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

#[cfg(feature = "python")]
use pyo3::prelude::*;

#[cfg(target_arch = "wasm32")]
pub mod app;

pub mod algorithms;

#[cfg(feature = "ai")]
pub mod ai;

#[cfg(feature = "app3d")]
pub mod app3d;

pub mod export;
pub mod importer;
pub mod railway_algorithms;
pub mod railway_api_client;
pub mod railway_model;
pub mod railway_objects;
pub mod simulation;
pub mod statistics;
pub mod types;

#[cfg(feature = "python")]
pub mod python;
#[cfg(feature = "python")]
pub use self::python::*;

#[cfg(test)]
pub mod tests;

/// This prelude re-exports the most commonly used items from the library.
pub mod prelude {
    #[cfg(target_arch = "wasm32")]
    pub use super::app::*;
    pub use super::export::*;
    pub use super::importer::{OverpassImporter, RailwayGraphImporter};
    pub use super::railway_api_client::overpass_api_client;
    pub use super::railway_api_client::{OverpassApiClient, RailwayApiClient};
    pub use super::railway_model::*;
}
