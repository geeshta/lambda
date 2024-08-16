mod alpha;
mod ast;
mod beta;
mod lexer;
mod parser;
mod substitution;
mod variables;
use std::fmt;

use ast::ast::AST;
use beta::{BetaReduction, EvalOrder};

fn main() {
    print_tree("($x, y -> x y) ($c, d -> c d) a");
    print_tree("a b c");
}

fn print_tree(input: &str) {
    let tokens = lexer::tokenize(input).unwrap();
    let tree = parser::parse(&tokens);
    println!("{:?}", tree)
}
