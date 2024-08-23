use std::fmt;
use std::os::raw::c_int;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ObjectHandle {
    pub fd: c_int,
    pub offset: usize,
    pub size: usize,
}

impl ObjectHandle {
    pub fn new(fd: c_int, offset: usize, size: usize) -> Self {
        ObjectHandle { fd, offset, size }
    }
}

impl fmt::Debug for ObjectHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ObjectHandle(fd:{},offset:{},size:{})", self.fd, self.offset, self.size)
    }
}
