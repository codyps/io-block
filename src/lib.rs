extern crate io_at;

/**
 * GPT and most other partition-y things operate on items that have a known end & have a known "block size"
 *
 * This provides an abstraction over that
 */
trait BlockContainer : io_at::ReadAt + io_at::WriteAt {
    fn block_sz(&self) -> u64;
    fn block_ct(&self) -> u64;

    // fn write_block();
    // fn read_block();
}

/**
 * Treat a normal File as a BlockContainer
 */
struct BlockFile {
    file: io::File,
    block_sz: u64,
    block_ct: u64,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
