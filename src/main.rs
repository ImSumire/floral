mod lexer;
mod ast;
mod parser;
mod error;
mod transpiler;

use std::fs;

use lexer::{Token, Logos};
use parser::Parser;

use crate::transpiler::Transpiler;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = "example/main.fl";
    let output = "example/main.c";
    let source = fs::read_to_string(path).expect("Failed to read source file");

    let lexer = Token::lexer(&source);
    let mut parser = Parser::new(lexer);
    match parser.parse_function() {
        Ok(func) => {
            println!("{func:#?}");
            let c_code = Transpiler::transpile_function(&func);
            fs::write(output, c_code)?;
        },
        Err(e) => eprintln!("{}", e.pprint(&source, path)),
    }

    Ok(())
}
