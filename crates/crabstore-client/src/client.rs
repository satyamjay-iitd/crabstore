use std::path::PathBuf;

use log::debug;

use crabstore_common;
use crabstore_common::messages::messages;
use crabstore_common::messages::MessageCodec;
use crabstore_common::messages::Messages;
use futures::SinkExt;
use pyo3::exceptions as pyexceptions;
use pyo3::prelude::*;
use std::os::unix::net::UnixStream as SyncUnixStream;
use std::sync::Arc;
use tokio::io;
use tokio::net::UnixStream;
use tokio::sync::Mutex;
use tokio_stream::StreamExt;
use tokio_util::codec::Framed;

use crate::status;

#[pyclass]
pub struct CrabClient {
    socket_name: PathBuf,
    framed: Option<Arc<Mutex<Framed<UnixStream, MessageCodec>>>>,
}

#[pyclass]
#[derive(Clone)]
pub struct ObjectID(crabstore_common::objectid::ObjectId);

#[pymethods]
impl ObjectID {
    #[staticmethod]
    pub fn from_binary(binary: &[u8]) -> Self {
        ObjectID(crabstore_common::objectid::ObjectId::from_binary(binary))
    }
}

impl CrabClient {
    async fn send_request(&mut self, request: Messages) -> io::Result<()> {
        if let Some(framed) = &mut self.framed {
            let mut framed = framed.lock().await;
            framed.send(request).await?;
        } else {
            return Err(io::Error::new(
                io::ErrorKind::NotConnected,
                "Client is not connected",
            ));
        }
        Ok(())
    }

    async fn receive_response(&mut self) -> io::Result<Messages> {
        if let Some(framed) = &mut self.framed {
            let mut framed = framed.lock().await;
            return match framed.next().await {
                Some(Ok(msg)) => Ok(msg),
                Some(Err(err)) => Err(err),
                None => Err(io::Error::new(io::ErrorKind::NotFound, "")),
            };
        }
        Err(io::Error::new(
            io::ErrorKind::NotConnected,
            "Client is not connected",
        ))
    }
    async fn connect_(&mut self) -> PyResult<status::Status> {
        let stream = UnixStream::connect(&self.socket_name).await?;
        debug!(
            "Connection with server established on socket_path = {:?}",
            &self.socket_name
        );
        self.framed = Some(Arc::new(Mutex::new(Framed::new(stream, MessageCodec {}))));

        let request = Messages::ConnectRequest(messages::ConnectRequest {});
        self.send_request(request).await?;
        debug!("Sent CONNECTION request to the server");

        match self.receive_response().await {
            Ok(Messages::ConnectResponse(cr)) => {
                debug!("Connection response received {:?}", cr);
                Ok(status::Status::ok())
            }
            Ok(r) => {
                debug!("Invalid response received {:?}", r);
                Err(pyexceptions::PyValueError::new_err(
                    "Invalid response received from sever",
                ))
            }
            Err(_) => Err(pyexceptions::PyConnectionError::new_err("")),
        }
    }

    fn connect2(&mut self) -> PyResult<status::Status> {
        let stream = SyncUnixStream::connect(&self.socket_name)?;
        debug!(
            "Connection with server established on socket_path = {:?}",
            &self.socket_name
        );
        self.framed = Some(Arc::new(Mutex::new(Framed::new(stream, MessageCodec {}))));

        let request = Messages::ConnectRequest(messages::ConnectRequest {});
        self.send_request(request).await?;
        debug!("Sent CONNECTION request to the server");

        match self.receive_response().await {
            Ok(Messages::ConnectResponse(cr)) => {
                debug!("Connection response received {:?}", cr);
                Ok(status::Status::ok())
            }
            Ok(r) => {
                debug!("Invalid response received {:?}", r);
                Err(pyexceptions::PyValueError::new_err(
                    "Invalid response received from sever",
                ))
            }
            Err(_) => Err(pyexceptions::PyConnectionError::new_err("")),
        }
    }
    pub async fn create_(
        &mut self,
        oid: ObjectID,
        data_size: u64,
        metadata_size: u64,
    ) -> PyResult<status::Status> {
        let request = Messages::CreateRequest(messages::CreateRequest {
            object_id: oid.0.binary(),
            is_mutable: false,
            data_size,
            metadata_size,
            device_num: 0,
            try_immediately: false,
        });
        self.send_request(request).await?;
        debug!("Sent CREATE request to the server");

        match self.receive_response().await {
            Ok(Messages::CreateResponse(cr)) => {
                debug!("CREATE response received {:?}", cr);
                Ok(status::Status::ok())
            }
            Ok(r) => {
                debug!("Invalid response received {:?}", r);
                Err(pyexceptions::PyValueError::new_err(
                    "Invalid response received from sever",
                ))
            }
            Err(_) => Err(pyexceptions::PyConnectionError::new_err("")),
        }
    }
}

#[pymethods]
impl CrabClient {
    #[new]
    pub fn new(socket_name: PathBuf) -> Self {
        CrabClient {
            socket_name,
            framed: None,
        }
    }

    // pub fn connect(&mut self, py: Python) -> PyResult<&PyAny> {
    //     pyo3_asyncio::tokio::future_into_py(py, async move {
    //         tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    //         Ok(Python::with_gil(|py| py.None()))
    //     })
    // pyo3_asyncio::tokio::future_into_py(py, async {
    //     tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    //     // self.connect_();
    //     Ok(Python::with_gil(|py| py.None()))
    // })
    // let rt = tokio::runtime::Runtime::new().unwrap();
    // let status = rt.block_on(self.connect_());
    // rt.shutdown_background();
    // status
    // }

    pub fn connect(&mut self, py: Python<'_>) -> PyResult<&PyAny> {
        pyo3_asyncio::tokio::future_into_py(py, async move {
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;

            // Obtain the GIL after the await point
            Python::with_gil(|py| Ok(py.None()))
        })
    }

    pub fn create(
        &mut self,
        oid: ObjectID,
        data_size: u64,
        metadata_size: u64,
    ) -> PyResult<status::Status> {
        println!(";askdkasdffds");
        let rt = tokio::runtime::Runtime::new().unwrap();
        println!("q029 8340293 840=2340=");
        let x = rt.block_on(self.create_(oid, data_size, metadata_size));
        println!("q029 8340293 840=2340=");
        rt.shutdown_background();
        x
    }
}
