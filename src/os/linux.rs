// ioctl
//  BLKSSZGET - bdev_logical_block_size() - logical, int
//  BLKPBSZGET - bdev_physical_block_size() - physical, uint

//  BLKBSZGET - block_size() - ????, int
//  BLKBSZSET - ??? set block size ???

// align-offset
// discard-zero
// rotational

use std::ffi::{c_int, c_uint, c_ushort};
use std::fs::File;
use std::io;
use std::os::unix::fs::FileTypeExt;
use std::os::unix::io::{AsRawFd, FromRawFd, IntoRawFd, RawFd};

use nix::{ioctl_none, ioctl_read, ioctl_read_bad, ioctl_write_ptr_bad, request_code_none};

use super::super::*;

ioctl_read_bad! {blksectget, request_code_none!(0x12, 103), c_ushort}
ioctl_read_bad! {blksszget, request_code_none!(0x12, 104), c_int}
ioctl_read! {blkbszget, 0x12, 112, c_int}
ioctl_read! {blkgetsize64, 0x12, 114, u64}
ioctl_read_bad! {blkiomin, request_code_none!(0x12, 120), c_uint}
ioctl_read_bad! {blkioopt, request_code_none!(0x12, 121), c_uint}
ioctl_read_bad! {blkalignoff, request_code_none!(0x12, 122), c_int}
ioctl_read_bad! {blkpbszget, request_code_none!(0x12, 123), c_uint}
ioctl_read_bad! {blkdiscardzeroes, request_code_none!(0x12, 124), c_uint}
ioctl_write_ptr_bad! {blkdiscard, request_code_none!(0x12, 119), [u64; 2]}

ioctl_read_bad! {blkroget, request_code_none!(0x12, 93), c_int}
ioctl_write_ptr_bad! {blkroset, request_code_none!(0x12, 94), c_int}
ioctl_none! {blkflsbuf, 0x12, 97}

/*
#define BLKROSET   _IO(0x12,93)	/* set device read-only (0 = read-write) */
#define BLKROGET   _IO(0x12,94)	/* get read-only status (0 = read_write) */
#define BLKRRPART  _IO(0x12,95)	/* re-read partition table */
#define BLKGETSIZE _IO(0x12,96)	/* return device size /512 (long *arg) */
#define BLKFLSBUF  _IO(0x12,97)	/* flush buffer cache */
#define BLKRASET   _IO(0x12,98)	/* set read ahead for block device */
#define BLKRAGET   _IO(0x12,99)	/* get current read ahead setting */
#define BLKFRASET  _IO(0x12,100)/* set filesystem (mm/filemap.c) read-ahead */
#define BLKFRAGET  _IO(0x12,101)/* get filesystem (mm/filemap.c) read-ahead */
#define BLKSECTSET _IO(0x12,102)/* set max sectors per request (ll_rw_blk.c) */
#define BLKSECTGET _IO(0x12,103)/* get max sectors per request (ll_rw_blk.c) */
#define BLKSSZGET  _IO(0x12,104)/* get block device sector size */


#define BLKBSZGET  _IOR(0x12,112,size_t)
#define BLKBSZSET  _IOW(0x12,113,size_t)
#define BLKGETSIZE64 _IOR(0x12,114,size_t)	/* return device size in bytes (u64 *arg) */
#define BLKTRACESETUP _IOWR(0x12,115,struct blk_user_trace_setup)
#define BLKTRACESTART _IO(0x12,116)
#define BLKTRACESTOP _IO(0x12,117)
#define BLKTRACETEARDOWN _IO(0x12,118)
#define BLKDISCARD _IO(0x12,119)
#define BLKIOMIN _IO(0x12,120)
#define BLKIOOPT _IO(0x12,121)
#define BLKALIGNOFF _IO(0x12,122)
#define BLKPBSZGET _IO(0x12,123)
#define BLKDISCARDZEROES _IO(0x12,124)
#define BLKSECDISCARD _IO(0x12,125)
#define BLKROTATIONAL _IO(0x12,126)
#define BLKZEROOUT _IO(0x12,127)
#define BLKGETDISKSEQ _IOR(0x12,128,__u64)
*/

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

    pub fn ro(&self) -> io::Result<bool> {
        let mut c: c_int = 0;
        unsafe { blkroget(self.as_raw_fd(), &mut c) }
            .map_err(|e| io::Error::from_raw_os_error(e as i32))?;

        Ok(c != 0)
    }

    pub fn block_io_min(&self) -> Result<u32> {
        let mut c: c_uint = 0;
        unsafe { blkiomin(self.as_raw_fd(), &mut c) }
            .map_err(|e| io::Error::from_raw_os_error(e as i32))?;

        Ok(c)
    }
}

impl BlockSize for BlockDev {
    fn block_size_logical(&self) -> Result<u64> {
        let mut c: c_int = 0;
        unsafe { blksszget(self.as_raw_fd(), &mut c) }
            .map_err(|e| io::Error::from_raw_os_error(e as i32))?;

        Ok(c as u64)
    }

    fn block_count(&self) -> Result<u64> {
        let mut c: u64 = 0;
        unsafe { blkgetsize64(self.as_raw_fd(), &mut c) }
            .map_err(|e| io::Error::from_raw_os_error(e as i32))?;

        Ok(c)
    }

    fn block_size_physical(&self) -> Result<u64> {
        let mut c: c_uint = 0;
        unsafe { blkpbszget(self.as_raw_fd(), &mut c) }
            .map_err(|e| io::Error::from_raw_os_error(e as i32))?;

        Ok(c as u64)
    }
}
