use std::fs::File;
use std::io::Read;
use anyhow::Result;

/// Read problem input from file
pub fn input_from_file(filename: &str) -> Result<String> {
    let mut f = File::open(filename)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    Ok(contents.trim().to_owned())
}