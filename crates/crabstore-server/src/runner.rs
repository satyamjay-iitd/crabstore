use log::info;
use std::path::PathBuf;

use crate::allocator;
use crate::store;

pub struct Runner {
    socket_name: PathBuf,
    sys_memory: i64,
    mem_mapped_dir: PathBuf,

    store: store::CrabStore,
}

impl Runner {
    pub fn new(socket_name: PathBuf, sys_memory: i64, mem_mapped_dir: PathBuf) -> Runner {
        // TODO: Check if directory has enough space;
        let allocator = allocator::RamAllocator::new();
        let store = store::CrabStore::new(socket_name.clone(), allocator);

        Runner {
            socket_name,
            sys_memory,
            mem_mapped_dir,
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
