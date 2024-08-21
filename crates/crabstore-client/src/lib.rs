mod allocator;
pub mod client;
mod rust_2_py;
use pyo3::prelude::*;

#[pymodule]
fn crabstore_client(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<client::ObjectID>()?;
    m.add_class::<client::CrabClient>()
}
