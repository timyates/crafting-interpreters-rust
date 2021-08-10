use std::fs::{create_dir_all, File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

fn main() {
    // println!("cargo:rerun-if-changed=src/expr.txt");
    let f = File::open("src/expr.txt").unwrap();
    let f = BufReader::new(f);
    create_dir_all("src/lox/expressions").unwrap();
    let dest_path = Path::new("src/lox/expressions/types.rs");
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .open(dest_path)
        .unwrap();

    writeln!(file, "pub enum Literal {{\n\
       \x20   True, False, Nil, Number(f64), String(String)\n\
       }}\n").unwrap();

    writeln!(file, "pub enum Expr {{").unwrap();
    for file_line in f.lines() {
        let line = file_line.unwrap();
        let name = line.split('=').next().unwrap().to_string();
        writeln!(file, "    {} {{", name).unwrap();
        let types = line.split('=').nth(1).unwrap().to_string();
        for typ in types.split(',') {
            writeln!(file, "        {},", typ).unwrap();
        }
        writeln!(file, "    }},").unwrap();
    }
    writeln!(file, "}}").unwrap();
}