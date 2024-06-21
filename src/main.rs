mod ast;
mod lexer;
mod parser;
mod substitution;
use lexer::lexer::tokenize;
use parser::parser::parse;

use std::fs;
use std::io;

fn read_file(filename: &str) -> io::Result<String> {
    let content = fs::read_to_string(filename)?;
    Ok(content.chars().filter(|c| !c.is_whitespace()).collect())
}

fn main() {
    let (lhs, rhs) = ("$xy -> xzy", "$zy -> zzy");
    let (ltokens, rtokens) = (tokenize(lhs).unwrap(), tokenize(rhs).unwrap());
    println!("[LHS] Tokens: {:?}", ltokens);
    println!("[RHS] Tokens: {:?}", rtokens);
    let (lresult, rresult) = (parse(&ltokens), parse(&rtokens));
    match (lresult, rresult) {
        (Ok(last), Ok(rast)) => {
            println!("[LHS] AST: {:?}", last);
            println!("[RHS] AST: {:?}", rast);
            println!("Equal: {:?}", last == rast);
        }
        (Err(e), _) | (_, Err(e)) => {
            println!("Parsing error: {:?}", e)
        }
    }
}
