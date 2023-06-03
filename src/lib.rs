use std::io::Result;

/// Some writable and readable devices (disk drives, media cards) have a block size (number of bytes
/// that are read or written at a time) associated with them.
///
/// These devices also have a (relatively) fixed (or at least known) number of blocks that limits
/// their length. At least, they can't be appended to.

// TODO: consider if we should require ReadAt &/or WriteAt
// TODO: consider if we should provide (here or in a seperate trait) read/write methods on blocks
// of data.
// TODO: consider providing block_sz_physical?
pub trait BlockDev {
    /// The number of bytes in each logical block
    fn block_size_logical(&self) -> Result<u64>;

    /// The total number of logical blocks
    fn block_count(&self) -> Result<u64>;

    /// The number of bytes in each physical block
    ///
    /// This is only a best guess. Many devices do not report a physical block size or do not
    /// report an accurate physical block size. Results will vary, be wary.
    fn block_size_physical(&self) -> Result<u64>;

    // fn write_block();
    // fn read_block();
}

mod file;
pub use file::BlockFile;

pub mod os;
