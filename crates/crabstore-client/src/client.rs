use std::fmt;

use bytes::BytesMut;
use crabstore_common::messages::messages;
use crabstore_common::messages::MessageCodec;
use crabstore_common::messages::Messages;
use crabstore_common::objectid::ObjectId;
use crabstore_common::objecthandle::ObjectHandle;
use dlmalloc::Dlmalloc;

use log::debug;

use std::os::raw::c_int;

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
pub struct ObjectID(ObjectId);

impl ObjectID {
    pub fn from_binary(binary: &[u8]) -> Self {
        ObjectID(ObjectId::from_binary(binary))
    }
}

struct OidRecord {
    oidmap: std::collections::HashMap<ObjectId, ObjectHandle>,
}

impl OidRecord {
    pub fn new() -> Self {
        OidRecord {
            oidmap: std::collections::HashMap::new(),
        }
    }
    pub fn insert(
        &mut self,
        oid: ObjectId,
        fd: c_int,
        offset: usize,
        size: usize
    ) {
        assert!(self
            .oidmap
            .insert(oid, ObjectHandle::new(fd, offset, size))
            .is_none());
    }

    pub fn remove(
        &mut self,
        oid: ObjectId
    ) -> Option<ObjectHandle> {
        self.oidmap.remove(&oid)
    }

    pub fn get(&self, oid: &ObjectId) -> Option<ObjectHandle> {
        self.oidmap.get(&oid).cloned()
    }
}

pub struct CrabClient {
    socket_name: PathBuf,
    stream: Option<Mutex<UnixStream>>,
    allocator: Dlmalloc<allocator::UnixSHM>,
    oids: OidRecord,
}

impl CrabClient {
    fn handle_from_oid(&self, oid: &ObjectId) -> Option<ObjectHandle> {
        self.oids.get(&oid)
    }
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
                6 => {
                    let cr = messages::OidSealRequest::decode(src);
                    match cr {
                        Ok(cr) => Ok(Messages::OidSealRequest(cr)),
                        Err(_) => Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            "Message decoding failed",
                        )),
                    }
                }
                7 => {
                    let cr = messages::OidSealResponse::decode(src);
                    match cr {
                        Ok(cr) => Ok(Messages::OidSealResponse(cr)),
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

    fn seal_oid(&mut self, oid: ObjectID, handle: ObjectHandle) -> io::Result<bool> {
        let request = Messages::OidSealRequest(messages::OidSealRequest {
            object_id: oid.0.binary(),
            fd: handle.fd,
            offset: handle.offset as u64,
            size: handle.size as u64
        });
        self.send_request(request)?;

        debug!("Sent Oid Seal request to the server");
        match self.receive_response() {
            Ok(Messages::OidSealResponse(cr)) => {
                debug!("OidSeal response received {:?}", cr);
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
            oids: OidRecord::new(),
        }
    }

    pub fn connect(&mut self) -> Result<(),io::Error> {
        let stream = UnixStream::connect(&self.socket_name)?;
        self.stream = Some(Mutex::new(stream));
        debug!(
            "Connection with server established on socket_path = {:?}",
            &self.socket_name
        );

        let request = Messages::ConnectRequest(messages::ConnectRequest {});
        self.send_request(request)?;
        debug!("Sent CONNECTION request to the server");

        match self.receive_response() {
            Ok(Messages::ConnectResponse(cr)) => {
                debug!("Connection response received {:?}", cr);
                Ok(())
            }
            Ok(r) => {
                debug!("Invalid response received {:?}", r);
                Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid response received from server"))
            }
            Err(e) => Err(e)
        }
    }

    pub fn create<'a>(
        &mut self,
        oid: ObjectID,
        data_size: usize,
    ) -> Result<&mut [u8],&'static str> {
        if self.reserve_oid(oid.clone(), data_size as u64).is_err() {
            return Err("ObjectID not available");
        }

        unsafe {
            let ptr = self.allocator.malloc(data_size, 1);
            let (fd,offset) = self.allocator.get_allocator().get_fd_offset_for_ptr(ptr).expect("allocator returned a pointer that does not exist in underlying allocator records.");
            self.oids.insert(oid.0, fd, offset, data_size);
            Ok(slice::from_raw_parts_mut(ptr, data_size))
        }
    }

    pub fn seal(&mut self, oid: ObjectID) -> Result<(),String> {
        let handle = match self.handle_from_oid(&oid.0) {
            None => {
                return Err("Invalid ObjectID supplied!".to_string());
            }
            Some(h) => h
        };
        match self.seal_oid(oid, handle) {
            Ok(_) => Ok(()),
            Err(e) => {
                Err(format!("Error during sea: {}", e.to_string()))
            }
        }
    }
}
