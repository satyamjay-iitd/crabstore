use log::info;
use std::path::PathBuf;

use crate::store;

pub struct Runner {
    socket_name: PathBuf,
    sys_memory: i64,
    store: store::CrabStore,
}

impl Runner {
    pub fn new(socket_name: PathBuf, sys_memory: i64) -> Runner {
        // TODO: Check if directory has enough space;
        let store = store::CrabStore::new(socket_name.clone());

        Runner {
            socket_name,
            sys_memory,
            store,
        }
    }
    pub fn start(self) {
        info!(
            "Starting Crabstore: Listening on {:?}. System Memory = {}",
            self.socket_name, self.sys_memory
        );

        let runtime = tokio::runtime::Runtime::new().unwrap();

        runtime.block_on(async { self.store.start().await });
    }
}
