use std::env;
use std::fs::File;
use std::io::{self, BufReader};
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let args_len: usize = args.len();
    if args_len > 2 {
        println!("Usage: lox [script]");
    } else if args_len == 2 {
        run_file(& args[1]);
    } else {
        run_prompt();
    }
}

fn run_file(path: & String) {
    match File::open(path) {
        Err(e) => println!("{:?}", e) ,
        Ok(file) => {
            let mut buf_reader = BufReader::new(file);
            let mut s: String = String::from("");
            buf_reader.read_to_string(&mut s).unwrap();
            run(& s);
        }
    }
}

fn run_prompt() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut buf_reader = BufReader::new(stdin);
    loop {
        print!("> ");
        stdout.flush().unwrap();
        let mut line: String = String::from("");
        buf_reader.read_line(&mut line).unwrap();
        run(& line);
    }
}

fn run(s: &String) {
    println!("{}", s);
}
