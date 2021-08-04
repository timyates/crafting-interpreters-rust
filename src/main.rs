use std::env::args;
use std::fs::{read_to_string};
use std::io::{stdin, Write, BufRead};
use std::io;
use std::process::exit;

fn run(line: &String) {
    println!("{}", line)
}

fn run_file(path: &String) {
    run(&read_to_string(path).unwrap());
}

fn run_prompt() {
    let mut line = String::new();

    loop {
        print!("lox > ");
        io::stdout().flush().unwrap();
        stdin().lock().read_line(&mut line).unwrap();
        let trimmed = line.trim_end().to_string();
        if trimmed.len() == 0 { break; }
        run(&trimmed);
        line.clear()
    }
}

fn main() {
    let a: Vec<String> = args().skip(1).collect();
    if a.len() > 1 {
        eprintln!("Usage: lox [script]");
        exit(64)
    } else if a.len() == 0 {
        run_prompt()
    } else {
        run_file(a.get(0).expect("Arg with no arg?"))
    }
}