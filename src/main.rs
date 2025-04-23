use std::{fs::File, io};
use std::io::Write;
use datetime::LocalDate;
use datetime::convenience::Today;

fn main() -> Result<(), std::io::Error> {
    // Проверка открытия файла kv.db
    let result = File::open("kv.db");

    match result {
        Ok(_) => println!("File is opened"),
        Err(_) => {
            println!("[** ERROR **] -- The file is unavailable or missing");
            // Создание файла kv.db
            File::create("kv.db")?;
        }
    }

    print!("Введите параметр: ");
    io::stdout().flush().unwrap();

    let mut input_str: String = String::new();

    io::stdin()
        .read_line(&mut input_str)
        .expect("[** ERROR **] -- Incorrect input");

    match input_str.to_lowercase().trim() {
        "create" => println!("Ok -- create"),
        "read" => println!("Ok -- read"),
        "update" => println!("Ok -- update"),
        "delete" => println!("Ok -- delete"),
        _ => println!("[** ERROR **] -- The pattern doesn't match")
    }

    let today: LocalDate = LocalDate::today();
    println!("LocalDate: {:?}", today);

    Ok(())
}