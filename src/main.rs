mod error;
mod expr;
mod parser;
mod scanner;
mod token;
mod tokentype;
// mod ast_printer;
mod environment;
mod interpreter;
mod lox_function;
mod resolver;
mod stmt;
// use crate::ast_printer::AstPrinter;
use crate::interpreter::Interpreter;
use crate::parser::Parser;
use crate::resolver::Resolver;
use crate::scanner::Scanner;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let args_len: usize = args.len();
    if args_len > 2 {
        println!("Usage: lox [script]");
    } else if args_len == 2 {
        run_file(&args[1]);
    } else {
        run_prompt();
    }
}

fn run_file(path: &String) {
    match File::open(path) {
        Err(e) => println!("{:?}", e),
        Ok(file) => {
            let mut buf_reader = BufReader::new(file);
            let mut s: String = String::from("");
            buf_reader.read_to_string(&mut s).unwrap();
            run(&s);
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
        run(&line);
    }
}

fn run(source: &String) {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();
    // for token in tokens {
    //     println!("{}", token);
    // }
    let mut parser = Parser::new(tokens);
    let statements = parser.parse().expect("Error");
    // let printer = AstPrinter {};
    // println!("{}", printer.print(&expr));
    let mut interpreter = Interpreter::new();
    let mut resolver = Resolver::new(&mut interpreter);
    resolver.resolves(&statements);
    interpreter.interpret(statements);
}
