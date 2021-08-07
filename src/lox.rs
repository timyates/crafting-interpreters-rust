mod scanner;

use std::fs::{read_to_string};
use std::io::{stdin, Write, BufRead};
use std::io;
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
        let tokens = scanner.scan_tokens();
        for token in tokens {
            println!("{:?}", token)
        }
    }

    fn error(&mut self, line: u32, message: &str) {
        self.report(line, "", message)
    }

    fn report(&mut self, line: u32, location: &str, message: &str) {
        eprintln!("[line {}] Error {} : {}", line, location, message);
        self.had_error = true
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
            if trimmed.len() == 0 { break; }
            self.run(&trimmed);
            self.had_error = false;
            line.clear()
        }
    }
}