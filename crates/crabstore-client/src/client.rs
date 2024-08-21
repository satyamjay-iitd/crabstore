use bytes::BytesMut;
use crabstore_common::messages::messages;
use crabstore_common::messages::MessageCodec;
use crabstore_common::messages::Messages;
use dlmalloc::Dlmalloc;

use log::debug;

use prost::Message;
use std::io;
use std::io::Read;
use std::io::Write;
use std::os::unix::net::UnixStream;
use std::path::PathBuf;
use std::sync::Mutex;

use crate::allocator;
use std::slice;
use tokio_util::codec::Encoder;


#[derive(Clone)]
pub struct ObjectID(crabstore_common::objectid::ObjectId);

impl ObjectID {
    pub fn from_binary(binary: &[u8]) -> Self {
        ObjectID(crabstore_common::objectid::ObjectId::from_binary(binary))
    }
}

pub struct CrabClient {
    socket_name: PathBuf,
    stream: Option<Mutex<UnixStream>>,
    allocator: Dlmalloc<allocator::UnixSHM>,
}

impl CrabClient {
    fn send_request(&mut self, request: Messages) -> Result<(), io::Error> {
        if let Some(stream_mutex) = &mut self.stream {
            let stream = stream_mutex.get_mut().unwrap();
            let mut mc = MessageCodec {};
            let mut b = BytesMut::new();
            mc.encode(request, &mut b)?;
            stream.write_all(b.as_mut())
        } else {
            Err(io::Error::new(
                io::ErrorKind::NotConnected,
                "Client is not connected",
            ))
        }
    }

    fn receive_response(&mut self) -> Result<Messages, io::Error> {
        if let Some(stream_mutex) = &mut self.stream {
            let stream = stream_mutex.get_mut().unwrap();

            let mut type_and_size = [0u8; 10];
            stream.read_exact(&mut type_and_size)?;

            let msg_type = u16::from_le_bytes([type_and_size[0], type_and_size[1]]);
            let msg_size = u64::from_le_bytes([
                type_and_size[2],
                type_and_size[3],
                type_and_size[4],
                type_and_size[5],
                type_and_size[6],
                type_and_size[7],
                type_and_size[8],
                type_and_size[9],
            ]);

            let mut src = BytesMut::from(&vec![0u8; msg_size as usize][..]);
            stream.read_exact(src.as_mut())?;

            // Based on the message type, decode the appropriate Protobuf message
            match msg_type {
                0 => {
                    let cr = messages::ConnectRequest::decode(src);
                    match cr {
                        Ok(cr) => Ok(Messages::ConnectRequest(cr)),
                        Err(_) => Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            "Message decoding failed",
                        )),
                    }
                }
                1 => {
                    let cr = messages::ConnectResponse::decode(src);
                    match cr {
                        Ok(cr) => Ok(Messages::ConnectResponse(cr)),
                        Err(_) => Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            "Message decoding failed",
                        )),
                    }
                }
                2 => {
                    let cr = messages::CreateRequest::decode(src);
                    match cr {
                        Ok(cr) => Ok(Messages::CreateRequest(cr)),
                        Err(_) => Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            "Message decoding failed",
                        )),
                    }
                }
                3 => {
                    let cr = messages::CreateResponse::decode(src);
                    match cr {
                        Ok(cr) => Ok(Messages::CreateResponse(cr)),
                        Err(_) => Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            "Message decoding failed",
                        )),
                    }
                }
                4 => {
                    let cr = messages::OidReserveRequest::decode(src);
                    match cr {
                        Ok(cr) => Ok(Messages::OidReserveRequest(cr)),
                        Err(_) => Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            "Message decoding failed",
                        )),
                    }
                }
                5 => {
                    let cr = messages::OidReserveResponse::decode(src);
                    match cr {
                        Ok(cr) => Ok(Messages::OidReserveResponse(cr)),
                        Err(_) => Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            "Message decoding failed",
                        )),
                    }
                }
                _ => {
                    // Unknown message type
                    Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "Unknown message type",
                    ))
                }
            }
        } else {
            Err(io::Error::new(
                io::ErrorKind::NotConnected,
                "Client is not connected",
            ))
        }
    }

    fn reserve_oid(&mut self, oid: ObjectID, size: u64) -> io::Result<bool> {
        let request = Messages::OidReserveRequest(messages::OidReserveRequest {
            object_id: oid.0.binary(),
            size,
        });
        self.send_request(request)?;

        debug!("Sent Oid Reserve request to the server");
        match self.receive_response() {
            Ok(Messages::OidReserveResponse(cr)) => {
                debug!("OidReserve response received {:?}", cr);
                println!("OidReserve response received {:?}", cr);
                Ok(true)
            }
            Ok(r) => {
                debug!("Invalid response received {:?}", r);
                Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Invalid response received from sever",
                ))
            }
            Err(err) => Err(err),
        }
    }
}

impl CrabClient {
    pub fn new(socket_name: PathBuf) -> Self {
        CrabClient {
            socket_name,
            stream: None,
            allocator: Dlmalloc::new_with_allocator(allocator::UnixSHM::new()),
        }
    }

    pub fn connect(&mut self) -> Result<(),String> {
        match UnixStream::connect(&self.socket_name) {
            Err(e) => {
                return Err(e.to_string())
            },
            Ok(stream) => {
                self.stream = Some(Mutex::new(stream));
            }
        };
        debug!(
            "Connection with server established on socket_path = {:?}",
            &self.socket_name
        );

        let request = Messages::ConnectRequest(messages::ConnectRequest {});
        match self.send_request(request) {
            Err(e) => return Err(e.to_string()),
                _ => {}
        };
        debug!("Sent CONNECTION request to the server");

        match self.receive_response() {
            Ok(Messages::ConnectResponse(cr)) => {
                debug!("Connection response received {:?}", cr);
                Ok(())
            }
            Ok(r) => {
                debug!("Invalid response received {:?}", r);
                Err("Invalid response received from sever".to_string())
            }
            Err(e) => Err(e.to_string())
        }
    }

    pub fn create<'a>(
        &mut self,
        oid: ObjectID,
        data_size: usize,
    ) -> Result<&[u8],&'static str> {
        if self.reserve_oid(oid, data_size as u64).is_err() {
            return Err("ObjectID not available");
        }

        unsafe {
            let ptr = self.allocator.malloc(data_size, 1);
            Ok(slice::from_raw_parts(ptr, data_size))
        }
    }
}
