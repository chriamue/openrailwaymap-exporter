use crate::prelude::overpass_api_client::OverpassApiClient;
use crate::railway_api_client::RailwayApiClient;

use pyo3::prelude::*;
use pythonize::pythonize;
use serde_json::Value;

#[pyfunction]
#[pyo3(signature = (area_name, url=None))]
fn fetch_by_area_name(
    py: Python<'_>,
    area_name: String,
    url: Option<String>,
) -> PyResult<Bound<'_, PyAny>> {
    let mut api_client = OverpassApiClient::new();

    pyo3_async_runtimes::tokio::future_into_py(py, async move {
        if let Some(url) = url {
            api_client.connect(&url).await.map_err(|err| {
                PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{}", err))
            })?;
        }
        let json_value: Value = api_client
            .fetch_by_area_name(&area_name)
            .await
            .map_err(|err| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{}", err)))?;
        Python::attach(|py| {
            pythonize(py, &json_value)
                .map(|bound| bound.unbind())
                .map_err(Into::into)
        })
    })
}

#[pymodule]
pub fn init_overpass_api_client(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(fetch_by_area_name, m)?)?;
    Ok(())
}
