use std::io::{self, Write};
use std::process;

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

enum InputState {
    Command,
    Statment,
}

fn get_result(line: String, op: InputState) {
    match op {
        InputState::Command => {
            if line == ".exit" {
                process::exit(1);
            } else {
                println!("Unrecognized command '{}'.", line);
            }
        }
        InputState::Statment => {
            if line.starts_with("insert") {
                println!("The is where we would do an insert.");
            } else if line == "select" {
                println!("This is where we would do a select.");
            } else {
                println!("Unrecognized keyword at start of '{}'.", line);
            }
        }
    }
}

fn print_prompt() {
    print!("db > ");
    io::stdout().flush().unwrap();
}

fn main() {
    let mut input_buffer = InputBuffer::new();

    loop {
        print_prompt();
        input_buffer.read_input();

        if input_buffer.buffer.starts_with('.') {
            let buf_string = input_buffer.buffer.clone();
            get_result(buf_string, InputState::Command);
        } else {
            let buf_string = input_buffer.buffer.clone();
            get_result(buf_string, InputState::Statment);
        }
    }
}
