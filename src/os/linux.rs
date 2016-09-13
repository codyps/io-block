extern crate ioctls as ioctl;
// ioctl
//  BLKSSZGET - bdev_logical_block_size() - logical, int
//  BLKPBSZGET - bdev_physical_block_size() - physical, uint

//  BLKBSZGET - block_size() - ????, int
//  BLKBSZSET - ??? set block size ???

// align-offset
// discard-zero
// rotational
use std::fs::File;
use std::io;
use std::os::unix::io::{AsRawFd,IntoRawFd,FromRawFd,RawFd};
use std::os::unix::fs::FileTypeExt;
use std::os::raw::{c_int};
use super::super::*;

pub struct BlockDev {
    // TODO: consider generalizing for other AsRawFd types
    // TODO: consider just storing a RawFd instead of a File
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
    pub unsafe fn from_file_raw(i: File) -> BlockDev {
        BlockDev { inner: i }
    }

    pub fn from_file(i: File) -> io::Result<BlockDev> {
        let m = try!(i.metadata());
        if !m.file_type().is_block_device() {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Not a block device"));
        }

        Ok(unsafe { BlockDev::from_file_raw(i) })
    }

    pub fn ro(&self) -> io::Result<bool> {
        let mut c: c_int = 0;
        let r = unsafe { ioctl::blkroget(self.as_raw_fd(), &mut c) };
        if r < 0 {
            Err(io::Error::last_os_error())
        } else {
            Ok(c != 0)
        }
    }
}

impl BlockSize for BlockDev {
    fn block_size_logical(&self) -> Result<u64> {
        let mut c : c_int = 0;
        let r = unsafe { ioctl::blksszget(self.as_raw_fd(), &mut c) };
        if r < 0 {
            Err(Error::last_os_error())
        } else {
            Ok(c as u64)
        }
    }

    fn block_count(&self) -> Result<u64> {
        let mut c: u64 = 0;
        let r = unsafe { ioctl::blkgetsize64(self.as_raw_fd(), &mut c) };
        if r < 0 {
            Err(Error::last_os_error())
        } else {
            Ok(c as u64)
        }
    }
}
