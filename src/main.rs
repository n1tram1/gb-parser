
use std::env;
use std::path;
use std::fs;
use std::io::Read;

use parser;

fn print_disass(path: &path::Path) {
    println!("printing {}", path.display());
    let mut file = fs::File::open(path).unwrap();

    let mut buffer: Vec<u8> = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    let mut i = 0;
    while i < buffer.len() {
        let inst_res =  parser::parse(&buffer[i..]).map_err(|e| panic!("{}", e));
        if let Ok(inst) = inst_res {
            i += inst.get_instruction().size();
            println!("{}", inst);
        }
    }
}

fn main() {
    for path in env::args().skip(1) {
        print_disass(path::Path::new(&path));
    }
}
