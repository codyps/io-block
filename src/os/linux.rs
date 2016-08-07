extern crate ioctl;
// ioctl
//  BLKSSZGET - bdev_logical_block_size() - logical, int
//  BLKPBSZGET - bdev_physical_block_size() - physical, uint

//  BLKBSZGET - block_size() - ????, int
//  BLKBSZSET - ??? set block size ???

// align-offset
// discard-zero
// rotational
use std::io;

struct BlockDev {
    // TODO: consider generalizing for other AsRawFd types
    inner: io::File,
}

impl BlockDev {
    fn from(i: io::File) -> io::Result<BlockDev> {
        let m = try!(i.metadata());
        if !m.file_type().is_block_device() {
            return Err(io::Error::InvalidInput, "Not a block device");
        }

        Ok(BlocKDev { inner: i })
    }
}

impl BlockSize for BlockDev {
    fn block_size_logical(&self) -> Result<u64> {
        let c : ioctl::libc::c_int = 0;
        let r = unsafe { ioctl::blksszget(self.as_raw_fd(), &mut c) };
        if r < 0 {
            Err(Error::last_os_error())
        } else {
            Ok(c as u64)
        }
    }

    fn block_count(&self) -> Result<u64> {
        let c: ioctl::libc::uint64_t = 0;
        let r = unsafe { ioctl::blkgetsize64(self.as_raw_fd(), &mut c) };
        if r < 0 {
            Err(Error::last_os_error())
        } else {
            Ok(c as u64)
        }
    }
}
