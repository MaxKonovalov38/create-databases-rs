fn main() {
	// Скипуем первый аргумент
	let mut args = std::env::args().skip(1);
	let key = args.next().unwrap();
	let value = args.next().unwrap();

	// Создаем формат записи в ДБ
	let contents = format!("{}\t{}\n", key, value);
	std::fs::write("kv.db", contents).unwrap();

	println!("Key: {}, Value: {}.", key, value);
}