use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn get_lines() -> std::io::Lines<BufReader<File>> {
    let arg = env::args().nth(1).expect("Need file to read");
    let path = Path::new(&arg);
    let file = File::open(path).unwrap_or_else(|_| panic!("could not open {}", path.display()));
    BufReader::new(file).lines()
}
