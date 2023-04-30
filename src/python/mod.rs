use pyo3::prelude::*;

use crate::importer::overpass_importer::OverpassImporter;
use crate::importer::RailwayGraphImporter;
use crate::railway_model::RailwayGraph;

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
        self.inner.graph.node_count()
    }

    /// Get the number of edges in the railway graph.
    ///
    /// # Returns
    ///
    /// * The number of edges in the railway graph.
    fn edge_count(&self) -> usize {
        self.inner.graph.edge_count()
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
    overpass_api_client::init_overpass_api_client(py, m)?;
    Ok(())
}
