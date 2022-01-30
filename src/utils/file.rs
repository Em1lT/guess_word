use std::io::{BufRead, BufReader};
use std::path::Path;
use std::fs::File;

pub fn read_file () -> std::io::Lines<std::io::BufReader<std::fs::File>> {
    let path = Path::new("lists/words.txt");
    let file = BufReader::new(File::open(&path).expect("Unable to open file"));
    file.lines()
}

