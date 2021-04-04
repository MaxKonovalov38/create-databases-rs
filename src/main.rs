use std::collections::HashMap;

#[derive(Debug)]
struct Database {
	inner: HashMap<String, String>,
}

impl Database {
	// add code here
	fn new() -> Result<Database, std::io::Error> {
		// TODO: проверить наличие ключа в БД
		let mut inner = HashMap::new();
		
		// let contents = std::fs::read_to_string("kv.bd")?
		let contents = match std::fs::read_to_string("kv.bd") {
			Ok(c) => c,
			Err(error) => return Err(error),
		};

		for line in contents.lines() {
			let chunks: Vec<&str> = line.split('\t').collect();

			if chunks.len() != 2 {
				todo!("Return Error!");
			}

			let key = chunks[0];
			let value = chunks[1];

			inner.insert(key.to_owned(), value.to_owned());
		}

		Ok(Database {
			inner: inner
		})
	}
}

fn main() {
	// Скипуем первый аргумент
	let mut args = std::env::args().skip(1);

	let key = args.next().unwrap();
	let value = args.next().unwrap();

	let _db = Database::new();

	// Создаем формат записи в ДБ
	let contents = format!("{}\t{}\n", key, value);
	std::fs::write("kv.db", contents).unwrap();

	println!("Key: {}, Value: {}.", key, value);
}