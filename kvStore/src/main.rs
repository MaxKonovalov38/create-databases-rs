use std::env;
use std::fs::File;
use std::io;
use std::process;

mod create;
mod delete;
mod read;
mod update;

/// Вызов сообщения о версии программы
fn version_out() {
	let version = String::from(env!("CARGO_PKG_VERSION"));
	println!("верия kvStore: v{}", version);
}

/// Вывод полной документации программы
fn help_out() {
	let mut text_hp = String::from(env!("CARGO_PKG_NAME"));
	text_hp.push_str(" (v");
	text_hp.push_str(env!("CARGO_PKG_VERSION"));
	text_hp.push_str("), ");
	text_hp.push_str(env!("CARGO_PKG_DESCRIPTION"));
	println!("{}\n", text_hp);
	title_out();
}

/// Краткое пояснение работы программы
fn title_out() {
	println!("Пример выполнения:\n\tcargo run -- [param]");
	println!("\nПараметры:\n\t'-v, --version'  Версия программы\n\t'-h, --help'  Документация программы");
	println!("\t'create'  Процесс добавления новой информации\n\t'read'  Операция получения данных, уже хранящихся в системе\n\t'update'  Обеспечивает изменения данных (value)\n\t'delete'  Процесс удаления ненужных данных из системы\n
    ");
}

/// Проверка открытия файла kv.db
fn checking_availability(file_name: &str) -> Result<(), std::io::Error> {
    let result = File::open(file_name);

    match result {
        Ok(_) => println!("File is opened"),
        Err(_) => {
            eprintln!("[** ERROR **] -- The file is unavailable or missing");
            // Создание файла kv.db
            File::create("kv.db").expect("[** ERROR **] -- Mot create file");
            println!("[** OK **] -- Create file");
        }
    }

    Ok(())
}

/// Считываем ввод с клавиатуры
fn getting_parameters() -> (String, String) {
    // Считываем значение - ключ
    let mut param_key: String = String::new();
    io::stdin()
        .read_line(&mut param_key)
        .expect("[** ERROR **] -- Param is not valid");

    // Считываем значение - значение
    let mut param_value: String = String::new();
    io::stdin()
        .read_line(&mut param_value)
        .expect("[** ERROR **] -- Param is not valid");

    (param_key, param_value)
}

fn main() -> Result<(), std::io::Error> {
    let file_name = "kv.db";

    // Извлечение аргументов командной строки
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        // Выполнение программы подрузомевает: name_program argv
        title_out();
        process::exit(0x0100);
    } else if &args[1] == "--version" || &args[1] == "-v" {
        // Выводит сообщение о версии программы
        version_out();
        process::exit(0x0100);
    } else if &args[1] == "--help" || &args[1] == "-h" {
        // Выводит сообщение справка
        help_out();
        process::exit(0x0100);
    }

    // Проверка аргумента параметра
    let input_str = &args[1];

    match input_str.to_lowercase().trim() {
        "create" => {
            let _ = checking_availability(file_name);
            let _ = create::main(getting_parameters(), file_name);
        }
        "read" => {
            let _ = checking_availability(file_name);
            read::main(file_name);
        }
        "update" => {
            let _ = checking_availability(file_name);
            let _ = update::main(getting_parameters(), file_name);
        }
        "delete" => {
            let _ = checking_availability(file_name);
            let _ = delete::main(getting_parameters(), file_name);
        }
        _ => eprintln!("[** ERROR **] -- The pattern doesn't match"),
    }

    Ok(())
}
