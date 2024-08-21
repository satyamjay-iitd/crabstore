pub mod pyclient;

mod rust_2_py;
use pyo3::prelude::*;

#[pymodule]
fn crabstore_client(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<pyclient::PyObjectID>()?;
    m.add_class::<pyclient::PyCrabClient>()
}
