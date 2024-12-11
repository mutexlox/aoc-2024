use std::env;
use std::fs::{read_to_string, File};
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn get_lines() -> std::io::Lines<BufReader<File>> {
    let arg = env::args().nth(1).expect("Need file to read");
    let path = Path::new(&arg);
    let file = File::open(path).unwrap_or_else(|_| panic!("could not open {}", path.display()));
    BufReader::new(file).lines()
}

pub fn get_all_input() -> String {
    let arg = env::args().nth(1).expect("Need file to read");
    read_to_string(&arg).unwrap_or_else(|_| panic!("Could not read {}", arg))
}
