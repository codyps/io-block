fn show_blk<T: AsRef<::std::path::Path>>(p: T) -> Result<(), ::std::io::Error> {
    let f = std::fs::File::open(p.as_ref())?;
    let b = io_block::os::BlockDev::from_file(f)?;
    println!(
        "{:?}: block-size-logical: {:?}",
        p.as_ref(),
        b.block_size_logical()
    );
    println!("{:?}: block-count: {:?}", p.as_ref(), b.block_count());
    println!(
        "{:?}: block-size-physical: {:?}",
        p.as_ref(),
        b.block_size_physical()
    );
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
