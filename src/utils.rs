use std::io::{self, Read, BufReader};
use std::fs::File;

pub fn read_file(path: &str) -> io::Result<Vec<u8>>
{
    let f = File::open(path)?;

    let mut reader = BufReader::new(f);
    let mut buffer = Vec::new();

    reader.read_to_end(&mut buffer)?;

    Ok(buffer)
}