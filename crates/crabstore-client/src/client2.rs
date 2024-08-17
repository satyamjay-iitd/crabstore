use crabstore_common::messages::messages;
use crabstore_common::messages::MessageCodec;
use crabstore_common::messages::Messages;
use log::debug;
use prost::bytes::BytesMut;
use prost::Message;
use pyo3::exceptions as pyexceptions;
use pyo3::prelude::*;
use std::io;
use std::io::Read;
use std::io::Write;
use std::os::unix::net::UnixStream;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Mutex;
use tokio_serde::Framed;

use crate::status;
use tokio_util::codec::{Decoder, Encoder};

#[pyclass]
pub struct CrabClient {
    socket_name: PathBuf,
    // framed: Option<Mutex<tokio_serde::Framed<UnixStream, Messages, BytesMut, MessageCodec>>>,
    // stream: Option<Arc<Mutex<tokio_serde::Framed<UnixStream, Messages, BytesMut, MessageCodec>>>>,
    stream: Option<UnixStream>,
}

impl CrabClient {
    fn send_request(&mut self, request: Messages) -> Result<(), io::Error> {
        if let Some(stream) = &mut self.stream {
            let mut mc = MessageCodec {};
            let mut b = BytesMut::new();
            mc.encode(request, &mut b)?;
            stream.write_all(b.as_mut())
        } else {
            return Err(io::Error::new(
                io::ErrorKind::NotConnected,
                "Client is not connected",
            ));
        }
    }

    fn receive_response(&mut self) -> Result<Messages, io::Error> {
        if let Some(stream) = &mut self.stream {
            let mut response = BytesMut::new();
            stream.read_to_end(response.as_mut());
            let mut mc = MessageCodec {};
            let optional_response = mc.decode(&mut response)?;
            if let Some(response) = optional_response {
                Ok(response)
            } else {
                Err(io::Error::new(
                    io::ErrorKind::NotConnected,
                    "Client is not connected",
                ))
            }
        } else {
            return Err(io::Error::new(
                io::ErrorKind::NotConnected,
                "Client is not connected",
            ));
        }
    }
}

#[pymethods]
impl CrabClient {
    #[new]
    pub fn new(socket_name: PathBuf) -> Self {
        CrabClient {
            socket_name,
            stream: None,
        }
    }

    pub fn connect(&mut self) -> PyResult<status::Status> {
        let stream = UnixStream::connect(&self.socket_name)?;
        debug!(
            "Connection with server established on socket_path = {:?}",
            &self.socket_name
        );
        self.stream = Some(stream);

        let request = Messages::ConnectRequest(messages::ConnectRequest {});
        self.send_request(request)?;
        debug!("Sent CONNECTION request to the server");

        match self.receive_response() {
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
}
