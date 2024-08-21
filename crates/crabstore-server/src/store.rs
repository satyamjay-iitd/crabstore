use crabstore_common::messages::messages;
use crabstore_common::messages::MessageCodec;
use crabstore_common::messages::Messages;
use futures::SinkExt;
use log::{debug,warn,error,info};
use std::io;
use std::path::Path;
use std::path::PathBuf;
use tokio::net::{UnixListener, UnixStream};
use tokio::signal;
use tokio_stream::StreamExt;
use tokio_util::codec::Framed;

pub struct CrabStore {
    socket_path: PathBuf,
}

impl CrabStore {
    pub fn new(socket_path: PathBuf) -> Self {
        CrabStore { socket_path }
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
                    tokio::spawn(async move {
                        handle_client(stream).await.expect("Error Happened during handling client");
                    });
                }
                _ = signal::ctrl_c() => {
                    info!("Shutting down the server");
                    drop(listener);
                    self.cleanup().await;
                    break;
                }
            }
        }

        Ok(())
    }

    pub async fn cleanup(&self) -> io::Result<()> {
        // remove the created socket
        if Path::new(&self.socket_path).exists() {
            info!("Removing socket at path {:?}", self.socket_path);
            std::fs::remove_file(&self.socket_path)
        } else {
            warn!("Socket {:?} does not exist!  This is unexpected!", self.socket_path);
            Ok(())
        }
    }
}

async fn handle_client(stream: UnixStream) -> io::Result<()> {
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
            Ok(Messages::OidReserveRequest(_cr)) => {
                debug!("Connect request received.");
                let response =
                    Messages::OidReserveResponse(messages::OidReserveResponse { oid_state: 0 });
                framed.send(response).await?;
            }
            Ok(invalid_request) => {
                error!("Invalid Request = {:?}", invalid_request);
            }
            Err(e) => {
                error!("error on decoding from socket; error = {:?}", e);
            }
        }
    }
    Ok(())
}
