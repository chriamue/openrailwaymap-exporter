//! This module provides Python bindings for importing and interacting with railway graphs.
//!
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use pythonize::pythonize;

use crate::importer::overpass_importer::OverpassImporter;
use crate::importer::RailwayGraphImporter;
use crate::railway_model::railway_graph::RailwayGraphExt;
use crate::railway_model::RailwayGraph;
use crate::types::{EdgeId, NodeId};

mod overpass_api_client;

/// A Python wrapper for the OverpassImporter struct.
#[pyclass]
pub struct PyOverpassImporter {}

#[pymethods]
impl PyOverpassImporter {
    /// Create a new PyOverpassImporter instance.
    #[new]
    fn new() -> Self {
        PyOverpassImporter {}
    }

    /// Import railway graph data from a JSON string.
    ///
    /// # Arguments
    ///
    /// * `input` - A JSON string containing railway graph data.
    ///
    /// # Returns
    ///
    /// * A PyRailwayGraph instance containing the imported railway graph data.
    fn import_graph(&self, input: &str) -> PyResult<PyRailwayGraph> {
        let json_value: serde_json::Value = serde_json::from_str(input)
            .map_err(|err| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{}", err)))?;

        let railway_graph = OverpassImporter::import(&json_value)
            .map_err(|err| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{}", err)))?;

        Ok(PyRailwayGraph {
            inner: railway_graph,
        })
    }
}

/// Export a PyRailwayGraph to an SVG string.
///
/// This function generates an SVG representation of the given PyRailwayGraph.
///
/// # Arguments
///
/// * `graph` - A reference to the PyRailwayGraph to be exported.
///
/// # Returns
///
/// * A PyResult containing a String of the SVG representation of the graph, or an error if the
///   conversion failed.
#[pyfunction]
pub fn export_svg(graph: &PyRailwayGraph) -> PyResult<String> {
    Ok(crate::export::generate_svg_string(&graph.inner).unwrap())
}

/// A Python wrapper for the RailwayGraph struct.
#[pyclass]
pub struct PyRailwayGraph {
    inner: RailwayGraph,
}

#[pymethods]
impl PyRailwayGraph {
    /// Get the number of nodes in the railway graph.
    ///
    /// # Returns
    ///
    /// * The number of nodes in the railway graph.
    fn node_count(&self) -> usize {
        self.inner.physical_graph.graph.node_count()
    }

    /// Get the number of edges in the railway graph.
    ///
    /// # Returns
    ///
    /// * The number of edges in the railway graph.
    fn edge_count(&self) -> usize {
        self.inner.physical_graph.graph.edge_count()
    }

    /// Get a node by its ID from the railway graph.
    ///
    /// # Arguments
    ///
    /// * `node_id` - The ID of the node to retrieve.
    ///
    /// # Returns
    ///
    /// * An optional `RailwayNode` instance if the node with the specified ID is found.
    fn get_node_by_id(&self, node_id: NodeId) -> Option<PyObject> {
        Some(Python::with_gil(|py| {
            pythonize(py, &self.inner.get_node_by_id(node_id).unwrap())
                .unwrap()
                .to_object(py)
        }))
    }

    /// Get an edge by its ID from the railway graph.
    ///
    /// # Arguments
    ///
    /// * `edge_id` - The ID of the edge to retrieve.
    ///
    /// # Returns
    ///
    /// * An optional `RailwayEdge` instance if the edge with the specified ID is found.
    fn get_edge_by_id(&self, edge_id: EdgeId) -> Option<PyObject> {
        Some(Python::with_gil(|py| {
            pythonize(py, &self.inner.get_edge_by_id(edge_id))
                .unwrap()
                .to_object(py)
        }))
    }
}

/// Initialize the openrailwaymap_exporter Python module.
///
/// # Arguments
///
/// * `_py` - The Python interpreter state.
/// * `m` - The Python module object.
///
/// # Returns
///
/// * A PyResult indicating the success or failure of the module initialization.
#[pymodule]
fn openrailwaymap_exporter(py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyOverpassImporter>()?;
    m.add_class::<PyRailwayGraph>()?;
    m.add_function(wrap_pyfunction!(export_svg, m)?)?;
    overpass_api_client::init_overpass_api_client(py, m)?;
    Ok(())
}
