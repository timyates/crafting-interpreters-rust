mod lox;

use std::env::args;
use std::process::exit;

fn main() {
    let a: Vec<String> = args().skip(1).collect();
    let mut lox = lox::fresh();
    if a.len() > 1 {
        eprintln!("Usage: lox [script]");
        exit(64)
    } else if a.len() == 0 {
        lox.run_prompt()
    } else {
        lox.run_file(a.get(0).expect("Arg with no arg?"))
    }
}