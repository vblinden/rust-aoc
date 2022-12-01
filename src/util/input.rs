use std::fs;

pub fn file(filename: &str) -> String {
    return fs::read_to_string(filename).expect("Can't find input file: {filename}");
}
