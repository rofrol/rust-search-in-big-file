use memmap::MmapOptions;
use std::fs::File;
use std::io::Write;
fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    println!("Hello, world!");
    // open file
    // search for string
    let file = File::open("README.md")?;
    let mmap = unsafe { MmapOptions::new().map(&file)? };
    assert_eq!(b"# rust-s", &mmap[0..8]);
    Ok(())
}
