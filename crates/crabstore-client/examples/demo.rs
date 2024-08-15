use std::path::PathBuf;

use crabstore_client::CrabClient;

#[tokio::main]
async fn main() {
    let mut client = CrabClient::new(PathBuf::from("sock"));
    client.connect().await.expect("Couldn't send data");
}
