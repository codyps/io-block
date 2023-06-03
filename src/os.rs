use crate::BlockSize;
use cfg_if::cfg_if;
use std::fs::File;
use std::io;

cfg_if! {
    if #[cfg(target_os = "linux")] {
        mod linux;
        use linux::BlockDev as OsBlockDev;
    } else if #[cfg(target_vendor = "apple")] {
        mod darwin;
        use darwin::BlockDev as OsBlockDev;
    } else {
        compile_error!("Unsupported OS");
    }

    // dragonflybsd: `getdisktabbyname`
    // freebsd: `getdiskbyname`
}

/// A block device on the target operating system
///
/// # Portability
///
/// - MacOS (darwin) will refuse to open block devices read-only if the file system is mounted,
///   either giving "Resource busy" or "Operation not permitted" depending on which disk (external or
///   internal) is opened. It is possible to use `IoRegistryEntryCreateCFProperties` to obtain
///   details while the device is mounted, but this is not implemented.
pub struct BlockDev {
    inner: OsBlockDev,
}

impl BlockDev {
    pub fn from_file(i: File) -> io::Result<BlockDev> {
        Ok(BlockDev {
            inner: OsBlockDev::from_file(i)?,
        })
    }
}

impl BlockSize for BlockDev {
    fn block_size_physical(&self) -> io::Result<u64> {
        self.inner.block_size_physical()
    }

    fn block_count(&self) -> io::Result<u64> {
        self.inner.block_count()
    }

    fn block_size_logical(&self) -> io::Result<u64> {
        self.inner.block_size_logical()
    }
}
