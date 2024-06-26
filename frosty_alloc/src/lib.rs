mod access;
mod allocator;
mod chunk;

pub use access::*;
pub use allocator::Allocator;

pub unsafe trait FrostyAllocatable {}
