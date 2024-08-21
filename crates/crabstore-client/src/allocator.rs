extern crate std;

use core::ptr;
use dlmalloc::Allocator;

use nix;
use std::os::fd::AsFd;
use std::string::ToString;

struct MMapRecord {
    addr2fdsize: std::collections::HashMap<*mut u8, (std::os::fd::OwnedFd, usize)>,
}

impl MMapRecord {
    pub fn new() -> MMapRecord {
        MMapRecord {
            addr2fdsize: std::collections::HashMap::new(),
        }
    }
    pub fn insert(
        &mut self,
        addr: core::ptr::NonNull<std::ffi::c_void>,
        fd: std::os::fd::OwnedFd,
        size: usize,
    ) {
        assert!(self
            .addr2fdsize
            .insert(addr.as_ptr() as *mut u8, (fd, size))
            .is_none());
    }

    pub fn remove(
        &mut self,
        addr: &core::ptr::NonNull<std::ffi::c_void>,
    ) -> Option<(std::os::fd::OwnedFd, usize)> {
        self.addr2fdsize.remove(&((*addr).as_ptr() as *mut u8))
    }
}

// convince the compiler that storing pointers in MMapRecord is fine
unsafe impl Send for MMapRecord {}

/// Allocator object for SHM allocation
pub struct UnixSHM {
    _priv: (),
    counter: i32,
    mmaps: MMapRecord,
}

impl UnixSHM {
    /// Allocate a new UnixSHM object
    pub fn new() -> UnixSHM {
        UnixSHM {
            _priv: (),
            counter: 0,
            mmaps: MMapRecord::new(),
        }
    }

    fn get_next_path(&mut self) -> std::string::String {
        self.counter += 1;

        let mut path = std::string::String::from("/dlmalloc-unixshm-");
        path.push_str(self.counter.to_string().as_str());
        path
    }
}

unsafe impl Allocator for UnixSHM {
    fn alloc(&mut self, size: usize) -> (*mut u8, usize, u32) {
        if size == 0 {
            return (ptr::null_mut(), 0, 0);
        }
        unsafe {
            let path = self.get_next_path();
            let fd = match nix::sys::mman::shm_open(
                path.as_str(),
                nix::fcntl::OFlag::O_CREAT | nix::fcntl::OFlag::O_RDWR,
                nix::sys::stat::Mode::S_IRWXU
                    | nix::sys::stat::Mode::S_IRWXG
                    | nix::sys::stat::Mode::S_IRWXO,
            ) {
                Ok(fd) => {
                    // unlink the file, we will retain the fd
                    match nix::sys::mman::shm_unlink(path.as_str()) {
                        Err(err) => {
                            std::eprintln!("shm_unlink(): {}", err);
                            return (ptr::null_mut(), 0, 0);
                        }
                        _ => (),
                    };
                    fd
                }
                Err(err) => {
                    std::eprintln!("shm_open(): {}", err);
                    return (ptr::null_mut(), 0, 0);
                }
            };

            if let Err(err) = nix::unistd::ftruncate(fd.as_fd(), size as i64) {
                std::eprintln!("ftruncate(): {}", err);
                return (ptr::null_mut(), 0, 0);
            }

            match nix::sys::mman::mmap(
                None,
                core::num::NonZeroUsize::new_unchecked(size),
                nix::sys::mman::ProtFlags::PROT_WRITE | nix::sys::mman::ProtFlags::PROT_READ,
                nix::sys::mman::MapFlags::MAP_SHARED,
                fd.as_fd(),
                0,
            ) {
                Ok(addr) => {
                    self.mmaps.insert(addr, fd, size);
                    (addr.as_ptr() as *mut u8, size, 0)
                }
                Err(err) => {
                    std::eprintln!("mmap(): {}", err);
                    (ptr::null_mut(), 0, 0)
                }
            }
        }
    }

    // #[cfg(target_os = "linux")]
    // fn remap(&self, ptr: *mut u8, oldsize: usize, newsize: usize, can_move: bool) -> *mut u8 {
    //     let flags = if can_move { libc::MREMAP_MAYMOVE } else { 0 };
    //     let ptr = unsafe { libc::mremap(ptr.cast(), oldsize, newsize, flags) };
    //     if ptr == libc::MAP_FAILED {
    //         ptr::null_mut()
    //     } else {
    //         ptr.cast()
    //     }
    // }

    // #[cfg(target_os = "macos")]
    fn remap(
        &mut self,
        _ptr: *mut u8,
        _oldsize: usize,
        _newsize: usize,
        _can_move: bool,
    ) -> *mut u8 {
        ptr::null_mut()
    }

    // #[cfg(target_os = "linux")]
    // fn free_part(&self, ptr: *mut u8, oldsize: usize, newsize: usize) -> bool {
    //     unsafe {
    //         let rc = libc::mremap(ptr.cast(), oldsize, newsize, 0);
    //         if rc != libc::MAP_FAILED {
    //             return true;
    //         }
    //         libc::munmap(ptr.add(newsize).cast(), oldsize - newsize) == 0
    //     }
    // }

    // #[cfg(target_os = "macos")]
    // fn free_part(&self, ptr: *mut u8, oldsize: usize, newsize: usize) -> bool {
    //     unsafe { libc::munmap(ptr.add(newsize).cast(), oldsize - newsize) == 0 }
    // }
    fn free_part(&mut self, _ptr: *mut u8, _oldsize: usize, _newsize: usize) -> bool {
        false
    }

    fn free(&mut self, ptr: *mut u8, size: usize) -> bool {
        let ptr_nn_cvoid = std::ptr::NonNull::new(ptr as *mut std::ffi::c_void)
            .expect("free(): input ptr was NULL");
        unsafe {
            let _to_be_dropped = match self.mmaps.remove(&ptr_nn_cvoid) {
                Some((fd, msize)) => {
                    if msize != size {
                        return false;
                    }
                    (fd, msize)
                }
                None => return false,
            };
            nix::sys::mman::munmap(ptr_nn_cvoid, size as libc::size_t).is_ok()
        }
    }

    fn can_release_part(&self, _flags: u32) -> bool {
        false
    }

    fn allocates_zeros(&self) -> bool {
        true // ftruncate zeros out the new region
    }

    fn page_size(&self) -> usize {
        4096 // not sure if it is meaningful here
    }
}
