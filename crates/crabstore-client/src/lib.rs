use std::io;
use std::path::PathBuf;

use crabstore_common::ObjectId;
use tokio::net::UnixStream;
use tokio_stream::StreamExt;
use tokio_util::codec::Framed;

pub struct CrabClient {
    socket_name: PathBuf,
    stream: Option<UnixStream>,
}

impl CrabClient {
    pub async fn new(socket_name: PathBuf) -> Self {
        CrabClient {
            socket_name,
            stream: None,
        }
    }

    pub async fn connect(&mut self) -> io::Result<()> {
        self.stream = Some(UnixStream::connect(&self.socket_name).await?);
        Ok(())
    }

    pub async fn create(
        &mut self,
        oid: &ObjectId,
        data_size: u64,
        metadata_size: u64,
    ) -> io::Result<()> {
        let stream = self.stream.expect("Call connect before creating objects");
        let mut framed = Framed::new(stream, crabstore_common::MessageCodec {});
        let request =
            crabstore_common::Messages::CreateRequest(crabstore_common::messages::CreateRequest {
                object_id: String::from(""),
            });
        framed.send(request).await?;

        Ok(())
    }
}

// pub fn add(left: u64, right: u64) -> u64 {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
