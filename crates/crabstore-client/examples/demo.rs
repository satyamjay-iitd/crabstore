use std::path::PathBuf;

use crabstore_client::client::CrabClient;

fn main()
{
    env_logger::init();
    let mut client = CrabClient::new(PathBuf::from("/tmp/sock_path"));
    match client.connect() {
        Err(s) => {
            eprintln!("Error connecting to server: {}", s);
            return;
        },
        _ => {}
    }
    let randbytes = [0; crabstore_common::objectid::UNIQUE_ID_SIZE];
    let obj_id = crabstore_client::client::ObjectID::from_binary(&randbytes);
    let arr = match client.create(obj_id.clone(), 20) {
        Err(e) => {
            eprintln!("Error create object: {}", e);
            return;
        }
        Ok(arr) => arr
    };
    arr[0] = 10;
    match client.seal(obj_id) {
        Err(e) => {
            eprintln!("Error seal object: {}", e);
            return;
        }
        Ok(_) => {}
    }
}
