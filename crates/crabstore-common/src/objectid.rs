use std::fmt;
use std::fmt::Write;
use std::hash::{Hash, Hasher};
pub const UNIQUE_ID_SIZE: usize = 20; // or whatever kUniqueIDSize is in your C++ code

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ObjectId {
    id: [u8; UNIQUE_ID_SIZE],
}

impl ObjectId {
    pub fn from_binary(binary: &[u8]) -> Self {
        let mut id = [0u8; UNIQUE_ID_SIZE];
        id.copy_from_slice(&binary[..UNIQUE_ID_SIZE]);
        ObjectId { id }
    }

    pub fn data(&self) -> &[u8] {
        &self.id
    }

    pub fn mutable_data(&mut self) -> &mut [u8] {
        &mut self.id
    }

    pub fn binary(&self) -> Vec<u8> {
        self.id.to_vec()
    }

    pub fn hex(&self) -> String {
        self.id.iter().fold(String::new(), |mut output, b| {
            let _ = write!(output, "{b:02x}");
            output
        })
    }

    pub fn hash(&self) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        let mut hasher = DefaultHasher::new();
        self.id.hash(&mut hasher);
        hasher.finish()
    }

    pub fn size() -> usize {
        UNIQUE_ID_SIZE
    }
}

impl fmt::Debug for ObjectId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "UniqueID({})", self.hex())
    }
}

impl Hash for ObjectId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
