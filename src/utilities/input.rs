use std::fs;

pub fn string(filename: &str) -> String {
    return fs::read_to_string(filename).expect("Can't find input file: {filename}");
}
