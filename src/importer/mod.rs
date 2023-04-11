//! module for importers
use crate::railway_model::RailwayGraph;
use anyhow::Result;
use serde_json::Value;
pub mod overpass_importer;
pub use overpass_importer::OverpassImporter;

/// `RailwayGraphImporter` is a trait for importing railway graph data from different formats/sources.
///
/// It provides a single method, `import`, which takes a reference to a `serde_json::Value` and returns
/// a `Result<RailwayGraph>`. Implementations of this trait are responsible for converting the input data
/// into a `RailwayGraph` object.
pub trait RailwayGraphImporter {
    /// Imports a railway graph from a given input data.
    ///
    /// This method receives a reference to a `serde_json::Value` and should return a `Result<RailwayGraph>`
    /// after processing the input data. In case of errors, an appropriate error type should be returned.
    ///
    /// # Arguments
    ///
    /// * `input` - A reference to a `serde_json::Value` representing the input data.
    ///
    /// # Returns
    ///
    /// A `Result<RailwayGraph>` containing the imported railway graph, or an error if the import fails.
    fn import(input: &Value) -> Result<RailwayGraph>;
}
