use bytes::BytesMut;
use crabstore_common::messages::messages;
use crabstore_common::messages::MessageCodec;
use crabstore_common::messages::Messages;
use dlmalloc::Dlmalloc;
use crabstore_client::client::{CrabClient,ObjectID};

use log::debug;

use prost::Message;

use pyo3::exceptions as pyexceptions;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyMemoryView;
use pyo3::Bound;

use std::io;
use std::io::Read;
use std::io::Write;
use std::os::unix::net::UnixStream;
use std::path::PathBuf;
use std::sync::Mutex;

use tokio_util::codec::Encoder;

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

impl PyCrabClient {
    fn send_request(&mut self, request: Messages) -> Result<(), io::Error> {
        self.crabclient.send_request(request)
    }

    fn receive_response(&mut self) -> Result<Messages, io::Error> {
        self.crabclient.receive_response()
    }

    fn reserve_oid(&mut self, oid: ObjectID, size: u64) -> io::Result<bool> {
        self.crabclient.reserve_oid(oid, size)
    }
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
