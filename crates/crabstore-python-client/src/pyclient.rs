use std::path::PathBuf;
use crabstore_client::client::{CrabClient,ObjectID};

use pyo3::exceptions as pyexceptions;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyMemoryView;
use pyo3::Bound;

use crate::rust_2_py::FromPtr;

#[pyclass]
#[derive(Clone)]
pub struct PyObjectID(ObjectID);

#[pymethods]
impl PyObjectID {
    #[staticmethod]
    pub fn from_binary(binary: &[u8]) -> Self {
        PyObjectID(ObjectID::from_binary(binary))
    }
}

#[pyclass]
pub struct PyCrabClient {
    crabclient: CrabClient,
}

#[pymethods]
impl PyCrabClient {
    #[new]
    pub fn new(socket_name: PathBuf) -> Self {
        PyCrabClient{ crabclient: CrabClient::new(socket_name) }
    }

    pub fn connect(&mut self) -> PyResult<()> {
        self.crabclient.connect().map_err(|err|
            pyexceptions::PyValueError::new_err(err.to_string()))
    }

    pub fn create<'a>(
        &mut self,
        py: Python<'a>,
        oid: PyObjectID,
        data_size: usize,
    ) -> PyResult<Bound<'a, PyMemoryView>> {
        unsafe {
            match self.crabclient.create(oid.0, data_size) {
                Ok(sl) =>  {
                    PyMemoryView::from_raw_ptr(py, sl.as_mut_ptr(), sl.len())
                }
                Err(es) => {
                    Err(PyValueError::new_err(es))
                }
            }
        }
    }
}
