use std::fs::File;
use std::io;
use std::io::Result;

mod file;
pub use file::BlockFile;

pub mod os;
use os::Disk as OsDisk;

/// Some writable and readable devices (disk drives, media cards) have a block size (number of bytes
/// that are read or written at a time) associated with them.
///
/// These devices also have a (relatively) fixed (or at least known) number of blocks that limits
/// their length. At least, they can't be appended to.

// TODO: consider if we should require ReadAt &/or WriteAt
// TODO: consider if we should provide (here or in a seperate trait) read/write methods on blocks
// of data.
// TODO: consider providing block_sz_physical?
pub trait BlockDevice {
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

/// A block device on the target operating system, typically representing a disk drive or media
///
/// # Portability
///
/// - MacOS (darwin) will refuse to open block devices read-only if the file system is mounted,
///   either giving "Resource busy" or "Operation not permitted" depending on which disk (external or
///   internal) is opened. It is possible to use `IoRegistryEntryCreateCFProperties` to obtain
///   details while the device is mounted, but this is not implemented.
pub struct Disk {
    inner: OsDisk,
}

impl Disk {
    pub fn from_file(i: File) -> io::Result<Disk> {
        Ok(Disk {
            inner: OsDisk::from_file(i)?,
        })
    }

    pub fn block_size_physical(&self) -> io::Result<u64> {
        self.inner.block_size_physical()
    }

    pub fn block_count(&self) -> io::Result<u64> {
        self.inner.block_count()
    }

    pub fn block_size_logical(&self) -> io::Result<u64> {
        self.inner.block_size_logical()
    }
}

impl BlockDevice for Disk {
    fn block_size_physical(&self) -> io::Result<u64> {
        self.block_size_physical()
    }

    fn block_count(&self) -> io::Result<u64> {
        self.block_count()
    }

    fn block_size_logical(&self) -> io::Result<u64> {
        self.block_size_logical()
    }
}
