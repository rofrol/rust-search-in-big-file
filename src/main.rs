use std::fs;
fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    println!("Hello, world!");
    // open file
    // search for string
    let contents = fs::read_to_string("README.md")?;
    println!("{}", contents);
    let found = contents.find("would");
    assert_eq!(found, Some(56));
    Ok(())
}
