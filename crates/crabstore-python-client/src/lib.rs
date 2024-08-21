mod allocator;
pub mod client;
mod status;
use pyo3::prelude::*;

#[pymodule]
fn crabstore_client(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<client::ObjectID>()?;
    m.add_class::<client::CrabClient>()?;
    m.add_class::<status::Status>()
}
