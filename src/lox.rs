mod scanner;

use std::fs::read_to_string;
use std::io;
use std::io::{stdin, BufRead, Write};
use std::process::exit;

pub struct Lox {
    had_error: bool,
}

pub fn fresh() -> Lox {
    Lox { had_error: false }
}

impl Lox {
    fn run(&mut self, line: &str) {
        let mut scanner = scanner::init(line);
        match scanner.scan_tokens() {
            Err(_) => self.had_error = true,
            Ok(tokens) => {
                for token in tokens {
                    println!("{:?}", token)
                }
            }
        }
    }

    pub fn run_file(&mut self, path: &str) {
        self.run(&read_to_string(path).unwrap());
        if self.had_error {
            exit(65);
        }
    }

    pub fn run_prompt(&mut self) {
        let mut line = String::new();
        loop {
            print!("lox > ");
            io::stdout().flush().unwrap();
            stdin().lock().read_line(&mut line).unwrap();
            let trimmed = line.trim_end().to_string();
            if trimmed.is_empty() {
                break;
            }
            self.run(&trimmed);
            self.had_error = false;
            line.clear()
        }
    }
}
