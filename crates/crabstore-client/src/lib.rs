pub mod client;
mod status;
use pyo3::prelude::*;

#[pymodule]
fn crabstore_client(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<client::CrabClient>()?;
    m.add_class::<client::ObjectID>()?;
    Ok(())
}
