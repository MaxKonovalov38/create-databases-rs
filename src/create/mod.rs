use std::fs::OpenOptions;
use std::io::Write;

pub fn main(params: (String, String), file_name: &str) -> Result<(), std::io::Error> {
    let text = format!("{}::{}\n", params.0.trim(), params.1.trim());

    //let mut file = File::open(&file_name)?;
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(file_name)
        .expect("msg");

    file.write_all(text.as_bytes())?;

    Ok(())
}
