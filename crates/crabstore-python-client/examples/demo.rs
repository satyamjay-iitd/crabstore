use std::path::PathBuf;

use crabstore_client::CrabClient;
use crabstore_common::objectid::ObjectId;

#[tokio::main]
async fn main() {
    let mut client = CrabClient::new(PathBuf::from("sock"));
    client.connect().await.expect("Couldn't send data");

    let oid = ObjectId::from_binary(b"00000000000000000000");
    client
        .create(&oid, 20, 20)
        .await
        .expect("Couldn't send data");
}
