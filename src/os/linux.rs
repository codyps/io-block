// ioctl
//  BLKSSZGET - bdev_logical_block_size() - logical, int
//  BLKPBSZGET - bdev_physical_block_size() - physical, uint

//  BLKBSZGET - block_size() - ????, int
//  BLKBSZSET - ??? set block size ???

// align-offset
// discard-zero
// rotational
use std::io;

mod ffi {

};

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
    fn block_size(&self) -> u64 {
        
    }
}
