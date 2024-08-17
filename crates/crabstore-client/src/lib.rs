pub mod client;
use std::{io, os::unix::net::UnixStream, path::PathBuf};
mod status;
use pyo3::prelude::*;

#[pyfunction]
fn sleep(socket_name: PathBuf) -> io::Result<()> {
    UnixStream::connect(socket_name)?;
    Ok(())
}

#[pymodule]
fn crabstore_client(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<client::CrabClient>()?;
    m.add_class::<client::ObjectID>()?;
    m.add_function(wrap_pyfunction!(sleep, m)?)
}
