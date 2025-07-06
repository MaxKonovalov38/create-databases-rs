use std::fs::File;
use std::io::Read;

pub fn main(file_name: &str) {
    let mut file = File::open(&file_name).expect("[** ERROR **]Failed to open file");
    let mut contents: String = String::new();

    file.read_to_string(&mut contents)
        .expect("[** ERROR **] -- Failed to read file");

    println!("File contents: {}", contents);
}
