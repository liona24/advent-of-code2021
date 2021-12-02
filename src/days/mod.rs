use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub mod day01;
pub mod day02;

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}