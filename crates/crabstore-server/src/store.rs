use crabstore_common::messages::messages;
use crabstore_common::messages::MessageCodec;
use crabstore_common::messages::Messages;
use futures::SinkExt;
use log::debug;
use log::info;
use std::io;
use std::path::Path;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use tokio::net::{UnixListener, UnixStream};
use tokio::signal;
use tokio_stream::StreamExt;
use tokio_util::codec::Framed;

use crate::allocator::RamAllocator;

pub struct CrabStore {
    socket_path: PathBuf,
    allocator: Arc<Mutex<RamAllocator>>,
}

impl CrabStore {
    pub fn new(socket_path: PathBuf, allocator: RamAllocator) -> Self {
        let allocator_mutex = Arc::new(Mutex::new(allocator));
        CrabStore {
            socket_path,
            allocator: allocator_mutex,
        }
    }

    pub async fn start(&self) -> io::Result<()> {
        // Remove the socket if it exists
        if Path::new(&self.socket_path).exists() {
            std::fs::remove_file(&self.socket_path)?;
        }
        let listener = UnixListener::bind(&self.socket_path)?;

        loop {
            tokio::select! {
                Ok((stream, _)) = listener.accept() => {
                    let allocator = self.allocator.clone();
                    tokio::spawn(async move {
                        handle_client(stream, allocator).await.expect("Error Happened during handling client");
                    });
                }
                _ = signal::ctrl_c() => {
                    info!("Shutting down the server");
                    break;
                }
            }
        }

        Ok(())
    }
}

async fn handle_client(stream: UnixStream, allocator: Arc<Mutex<RamAllocator>>) -> io::Result<()> {
    let mut framed = Framed::new(stream, MessageCodec {});

    while let Some(request) = framed.next().await {
        match request {
            Ok(Messages::CreateRequest(_cr)) => {
                debug!("Create request received.");
                let response = Messages::CreateResponse(messages::CreateResponse {
                    object_id: _cr.object_id,
                    retry_with_request_id: 0,
                    plasma_object: Some(messages::ObjectSpec {
                        segment_index: 0,
                        unique_fd_id: 0,
                        header_offset: 0,
                        data_offset: 0,
                        data_size: 0,
                        metadata_offset: 0,
                        metadata_size: 0,
                        allocated_size: 0,
                        fallback_allocated: false,
                        device_num: 0,
                        is_experimental_mutable_object: false,
                    }),
                    error: 0,
                    store_fd: 0,
                    unique_fd_id: 0,
                    mmap_size: 0,
                    ipc_handle: Some(messages::CudaHandle {
                        handle: vec![vec![0; 4]; 4],
                    }),
                });

                framed.send(response).await?;
            }
            Ok(Messages::ConnectRequest(_cr)) => {
                debug!("Connect request received.");
                let response = Messages::ConnectResponse(messages::ConnectResponse {
                    memory_capacity: 200,
                });
                framed.send(response).await?;
            }
            Ok(invalid_request) => {
                println!("Invalid Request = {:?}", invalid_request);
            }
            Err(e) => {
                println!("error on decoding from socket; error = {:?}", e);
            }
        }
    }
    Ok(())
}
