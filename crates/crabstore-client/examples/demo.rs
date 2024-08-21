use std::path::PathBuf;

use crabstore_client::client::CrabClient;

fn main()
{
    let mut client = CrabClient::new(PathBuf::from("/tmp/sock_path"));
    match client.connect() {
        Err(s) => {
            eprintln!("Error connecting to server: {}", s);
            return;
        },
        _ => {}
    }
    let randbytes = [0; crabstore_common::objectid::UNIQUE_ID_SIZE];
    let objId = crabstore_client::client::ObjectID::from_binary(&randbytes);
    match client.create(objId, 20) {
        Err(e) => {
            eprintln!("Error create object: {}", e);
            return;
        }
        Ok(arr) => {
            println!("Created an object of size {}", arr.len());
        }
    };
}
