use crabstore_common;
use log::info;
use std::io;
use std::path::Path;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use tokio::net::{UnixListener, UnixStream};
use tokio::signal;
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

async fn handle_client(
    mut stream: UnixStream,
    allocator: Arc<Mutex<RamAllocator>>,
) -> io::Result<()> {
    let mut data = vec![0; 4];

    let mut framed = Framed::new(stream, MessageCodec);

    while let Some(Ok(message)) = framed.next().await {
        println!("Received: {}", message);

        // You can use the allocator here if needed

        // Send a response
        framed.send("PONG".to_string()).await.unwrap();
    }
    Ok(())
}
