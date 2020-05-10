mod parser;

use std::env;
use std::path;
use std::fs;
use std::io::Read;

fn print_disass(path: &path::Path) {
    println!("printing {}", path.display());
    let mut file = fs::File::open(path).unwrap();

    let mut buffer: Vec<u8> = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    let instruction = parser::parse(&buffer).unwrap();
    println!("{:?}", instruction);
}

fn main() {
    for path in env::args().skip(1) {
        print_disass(path::Path::new(&path));
    }
}
