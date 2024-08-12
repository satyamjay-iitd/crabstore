pub struct Allocation {}
trait Allocator {
    fn allocate(size: i64) -> Allocation;
    fn free(allocation: Allocation);
    fn total_allocated() -> i64;
}
pub struct RamAllocator {}

impl RamAllocator {
    pub fn new() -> Self {
        RamAllocator {}
    }
}

impl Allocator for RamAllocator {
    fn allocate(size: i64) -> Allocation {
        todo!()
    }

    fn free(allocation: Allocation) {
        todo!()
    }

    fn total_allocated() -> i64 {
        todo!()
    }
}
