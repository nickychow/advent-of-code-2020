// missing_docs, missing_debug_implementations
#![warn(rust_2018_idioms)]
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub type Result<T> = std::result::Result<T, anyhow::Error>;

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn read_hashset<P>(filename: P) -> Result<HashSet<u64>>
where
    P: AsRef<Path>,
{
}
