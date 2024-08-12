use std::path::PathBuf;

use crabstore_client::CrabClient;

#[tokio::main]
async fn main() {
    let mut client = CrabClient::new(PathBuf::from("sock"))
        .await
        .expect("Couldn't initalize the client");
    client
        .send_message(b"PING")
        .await
        .expect("Couldn't send data");
}
