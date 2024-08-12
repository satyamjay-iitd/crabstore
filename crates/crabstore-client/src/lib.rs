use std::io;
use std::path::PathBuf;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::UnixStream;

pub struct CrabClient {
    stream: UnixStream,
}

impl CrabClient {
    pub async fn new(socket_name: PathBuf) -> io::Result<Self> {
        let stream = UnixStream::connect(socket_name).await?;
        Ok(CrabClient { stream })
    }

    pub async fn send_message(&mut self, msg: &[u8]) -> io::Result<()> {
        self.stream.writable().await?;
        self.stream.write_all(msg).await?;

        println!("Wrote {}", String::from_utf8(msg.to_vec()).expect(""));

        self.stream.readable().await?;
        let mut msg = vec![0; 1024];
        self.stream.read_to_end(&mut msg).await?;

        println!(
            "{}",
            String::from_utf8(msg.clone()).expect("Our bytes should be valid utf8")
        );
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
