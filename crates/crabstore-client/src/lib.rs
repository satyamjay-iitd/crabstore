// pub mod client;
pub mod client2;
mod status;
use pyo3::prelude::*;

#[pymodule]
fn crabstore_client(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<client2::ObjectID>()?;
    m.add_class::<client2::CrabClient>()
}
