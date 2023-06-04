use super::*;
use core::ops::{Deref, DerefMut};

/**
 * Wrap some type T in a block container
 */
pub struct BlockFile<T> {
    file: T,
    block_sz: u64,
    block_ct: u64,
}

impl<T> Deref for BlockFile<T> {
    type Target = T;
    fn deref(&self) -> &<Self as Deref>::Target {
        &self.file
    }
}

impl<T> DerefMut for BlockFile<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.file
    }
}

/*
impl<T> Into<T> for BlockFile<T> {
    fn into(self) -> T {
        self.file
    }
}
*/

impl<T> crate::BlockDev for BlockFile<T> {
    fn block_size_logical(&self) -> Result<u64> {
        Ok(self.block_sz)
    }

    fn block_count(&self) -> Result<u64> {
        Ok(self.block_ct)
    }

    fn block_size_physical(&self) -> Result<u64> {
        self.block_size_logical()
    }
}

impl<T> BlockFile<T> {
    pub fn new(file: T, block_sz: u64, block_ct: u64) -> BlockFile<T> {
        BlockFile {
            file,
            block_sz,
            block_ct,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn blk_file() {}
}
