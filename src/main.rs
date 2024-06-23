mod alpha;
mod ast;
mod beta;
mod lexer;
mod parser;
mod substitution;
mod variables;
use ast::ast::AST;
use beta::BetaReduction;

use std::fs;
use std::io;

fn read_file(filename: &str) -> io::Result<String> {
    let content = fs::read_to_string(filename)?;
    Ok(content.chars().filter(|c| !c.is_whitespace()).collect())
}

fn process_exprs(exprs: &[&str]) {
    for (expr) in exprs {
        println!("{}", expr);
        match AST::eval(&expr) {
            Ok(ast) => {
                println!("AST: {:?}", ast);
                let reduced = ast.reduce();
                println!("Reduced: {:?}", reduced);
                println!("{}", reduced.is_reducible);
            }
            Err(e) => println!("Error in expression '{}': {:?}", expr, e),
        }
    }
}

fn main() {
    let exprs = vec!["($name, last_name -> name X last_name) Stefan Foldesi"];
    process_exprs(&exprs);
}
