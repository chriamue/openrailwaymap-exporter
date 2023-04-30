use crate::prelude::overpass_api_client::OverpassApiClient;
use crate::railway_api_client::RailwayApiClient;

use pyo3::prelude::*;
use pythonize::pythonize;
use serde_json::Value;

#[pyfunction]
fn fetch_by_area_name<'a>(
    py: Python<'a>,
    area_name: String,
    url: Option<String>,
) -> PyResult<&'a PyAny> {
    let mut api_client = OverpassApiClient::new();

    pyo3_asyncio::tokio::future_into_py(py, async move {
        if let Some(url) = url {
            api_client.connect(&url).await.map_err(|err| {
                PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{}", err))
            })?;
        }
        let json_value: Value = api_client
            .fetch_by_area_name(&area_name)
            .await
            .map_err(|err| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{}", err)))?;
        Ok(Python::with_gil(|py| {
            pythonize(py, &json_value).unwrap().to_object(py)
        }))
    })
}

#[pymodule]
pub fn init_overpass_api_client(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(fetch_by_area_name, m)?)?;
    Ok(())
}
