extern crate io_block;
use io_block::BlockSize;

fn show_blk<T: AsRef<::std::path::Path>>(p: T) -> Result<(),::std::io::Error> {
        let f = try!(std::fs::File::open(p.as_ref()));
        let b = try!(io_block::os::linux::BlockDev::from_file(f));
        println!("{:?}: {:?} {:?} {:?}",
                 p.as_ref(), b.block_size_logical(),
                 b.block_count(),
                 b.block_size_physical());
        Ok(())
}

fn main() {
    for arg in std::env::args_os().skip(1) {
        match show_blk(arg) {
            Err(e) => println!("Error: {:?}", e),
            _ => {}
        }
    }
}
