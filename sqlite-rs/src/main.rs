use std::io::{self, Write};
use std::process;

// Структура для хранения ввода
struct InputBuffer {
    buffer: String,
}

impl InputBuffer {
    fn new() -> Self {
        InputBuffer {
            buffer: String::new(),
        }
    }

    fn read_input(&mut self) {
        self.buffer.clear();
        io::stdout().flush().unwrap();

        let bytes_read = io::stdin()
            .read_line(&mut self.buffer)
            .expect("error reading input!");

        if bytes_read == 0 {
            println!("error reading input!");
            process::exit(1);
        }

        if self.buffer.ends_with('\n') {
            self.buffer.pop();
            if self.buffer.ends_with('\r') {
                self.buffer.pop();
            }
        }
    }
}

fn print_prompt() {
    print!("db > ");
    io::stdout().flush().unwrap();
}

fn main() -> io::Result<()> {
    let mut input_buffer = InputBuffer::new();

    loop {
        print_prompt();
        input_buffer.read_input();

        if input_buffer.buffer == ".exit" {
            break;
        } else {
            println!("Unrecognized command '{}'.", input_buffer.buffer);
        }
    }

    Ok(())
}