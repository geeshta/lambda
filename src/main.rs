// mod alpha;
mod ast;
mod lexer;
mod parser;
// use alpha::renaming::Renaming;
use ast::term::Term;
use lexer::lexer::tokenize;
use parser::parser::parse;

use std::fs;
use std::io;

fn read_file(filename: &str) -> io::Result<String> {
    let content = fs::read_to_string(filename)?;
    Ok(content.chars().filter(|c| !c.is_whitespace()).collect())
}

fn main() {
    let input = "$f -> ($x -> f(xx)) ($y -> f(yy))";
    let tokens = tokenize(input).unwrap();
    println!("Tokens: {:?}", tokens);
    let parse_result = parse(&tokens);
    match parse_result {
        Ok(ast) => {
            println!("AST: {:?}", ast);
            // println!("Renamed: {:?}", ast.rename(Term::var('f'), Term::var('a')));
        }
        Err(e) => {
            println!("Parsing error: {:?}", e)
        }
    }
}
