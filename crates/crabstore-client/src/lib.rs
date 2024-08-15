use std::io;
use std::path::PathBuf;

use crabstore_common::messages::messages;
use crabstore_common::messages::MessageCodec;
use crabstore_common::messages::Messages;
use crabstore_common::objectid;
use crabstore_common::status;
use futures::SinkExt;
use tokio::net::UnixStream;
use tokio_stream::StreamExt;
use tokio_util::codec::Framed;

pub struct CrabClient {
    socket_name: PathBuf,
    framed: Option<Framed<UnixStream, MessageCodec>>,
}

impl CrabClient {
    pub fn new(socket_name: PathBuf) -> Self {
        CrabClient {
            socket_name,
            framed: None,
        }
    }

    pub async fn connect(&mut self) -> io::Result<status::Status> {
        let stream = UnixStream::connect(&self.socket_name).await?;
        self.framed = Some(Framed::new(stream, MessageCodec {}));

        let request = Messages::ConnectRequest(messages::ConnectRequest {});
        self.send_request(request).await?;

        if let Ok(Messages::ConnectResponse(_cr)) = self.receive_response().await {
            println!("{:?}", _cr);
            Ok(status::Status::ok())
        } else {
            Err(io::Error::new(io::ErrorKind::NotFound, ""))
        }
    }

    pub async fn create(
        &mut self,
        oid: &objectid::ObjectId,
        data_size: u64,
        metadata_size: u64,
    ) -> io::Result<status::Status> {
        let request = Messages::CreateRequest(messages::CreateRequest {
            object_id: oid.binary(),
            is_mutable: false,
            data_size,
            metadata_size,
            device_num: 0,
            try_immediately: false,
        });
        self.send_request(request).await?;
        if let Ok(Messages::CreateResponse(_cr)) = self.receive_response().await {
            Ok(status::Status::ok())
        } else {
            Err(io::Error::new(io::ErrorKind::NotFound, ""))
        }
    }

    async fn send_request(&mut self, request: Messages) -> io::Result<()> {
        if let Some(framed) = &mut self.framed {
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
}
