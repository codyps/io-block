// FIXME: permission failures occur opening disks even with `sudo`. Examine read/write open? Or
// look at using `IoRegistryEntryCreateCFProperties` to get the details as `diskutil` does.
//
// https://opensource.apple.com/source/xnu/xnu-7195.81.3/bsd/sys/disk.h.auto.html
//

//  * DKIOCGETPHYSICALBLOCKSIZE             get device's block size
// #define DKIOCGETPHYSICALBLOCKSIZE             _IOR('d', 77, uint32_t)
//  * DKIOCGETBLOCKSIZE                     get media's block size
// #define DKIOCGETBLOCKSIZE                     _IOR('d', 24, uint32_t)
// #define DKIOCGETBLOCKCOUNT                    _IOR('d', 25, uint64_t)

use crate::BlockSize;

use nix::ioctl_read;

use std::fs::File;
use std::io::{self, Result};
use std::os::unix::fs::FileTypeExt;
use std::os::unix::io::{AsRawFd, FromRawFd, IntoRawFd, RawFd};

ioctl_read! { dkiocgetphysicalblocksize, b'd', 77, u32 }
ioctl_read! { dkiocgetblocksize, b'd', 24, u32 }
ioctl_read! { dkiocgetblockcount, b'd', 25, u64 }

pub struct BlockDev {
    inner: File,
}

impl AsRawFd for BlockDev {
    fn as_raw_fd(&self) -> RawFd {
        self.inner.as_raw_fd()
    }
}

impl FromRawFd for BlockDev {
    unsafe fn from_raw_fd(fd: RawFd) -> BlockDev {
        BlockDev::from_file_raw(File::from_raw_fd(fd))
    }
}

impl IntoRawFd for BlockDev {
    fn into_raw_fd(self) -> RawFd {
        self.inner.into_raw_fd()
    }
}

impl BlockDev {
    /// Treat a file as a block device without checking
    ///
    /// # Safety
    ///
    /// `i` must refer to a block device file, otherwise the ioctls used by other functions may
    /// have undesired effects, including reading and writing memory unexpectedly.
    pub unsafe fn from_file_raw(i: File) -> BlockDev {
        BlockDev { inner: i }
    }

    pub fn from_file(i: File) -> io::Result<BlockDev> {
        let m = i.metadata()?;
        if !m.file_type().is_block_device() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Not a block device",
            ));
        }

        Ok(unsafe { BlockDev::from_file_raw(i) })
    }
}

impl BlockSize for BlockDev {
    fn block_size_logical(&self) -> Result<u64> {
        let mut c: u32 = 0;
        unsafe { dkiocgetblocksize(self.as_raw_fd(), &mut c) }
            .map_err(|e| io::Error::from_raw_os_error(e as i32))?;

        Ok(c as u64)
    }

    fn block_count(&self) -> Result<u64> {
        let mut c: u64 = 0;
        unsafe { dkiocgetblockcount(self.as_raw_fd(), &mut c) }
            .map_err(|e| io::Error::from_raw_os_error(e as i32))?;

        Ok(c)
    }

    fn block_size_physical(&self) -> Result<u64> {
        let mut c: u32 = 0;
        unsafe { dkiocgetphysicalblocksize(self.as_raw_fd(), &mut c) }
            .map_err(|e| io::Error::from_raw_os_error(e as i32))?;

        Ok(c as u64)
    }
}
