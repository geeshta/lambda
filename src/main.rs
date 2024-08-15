mod alpha;
mod ast;
mod beta;
mod lexer;
mod parser;
mod substitution;
mod variables;
use ast::ast::AST;
use beta::{BetaReduction, EvalOrder};
use std::fs;
use std::io;
use std::time::{Duration, Instant};

fn read_file(filename: &str) -> io::Result<String> {
    let content = fs::read_to_string(filename)?;
    Ok(content.chars().filter(|c| !c.is_whitespace()).collect())
}

fn eval_asts(asts: Vec<AST>, order: &EvalOrder) {
    for ast in asts {
        ast.reduce(Some(order));
    }
}

fn main() {
    const ITERATIONS: u32 = 10000;
    let exprs = vec![
        "((($x -> x) y) (($a -> a) y)) ((($b -> b) y) (($c -> c) y)) ((($d -> d) y) (($e -> e) y))",
        // Simple application with repetition
        "(($x -> x y) a) (($c -> c y) a)",
        // Nested applications with repetition
        "((($x -> x y) a) (($c -> c y) a)) ((($d -> d z) c) (d))",
        // Applications with alpha variants
        "((($x -> x y) a) (($c -> c y) a)) ((($d -> d y) a) (e))",
        // Applications with no repetitions
        "($x -> $y -> $z -> x y z) (a) (b) (c)",
        // Nested applications with deeper nesting and alpha variants
        "((($x -> $y -> x y) (a)) (($b -> $c -> b c) (a) (e))) ((($f -> $g -> f g) (a)) (i))",
        // Complex applications with mixed alpha variants and nested structure
        "((($x -> x z) a) (b)) ((($c -> c z) a) (e))",
        // Applications with no repetitions, involving deeper nesting
        "($x -> $y -> $z -> x (y z)) (a) (b) (c)",
        // Nested applications with alpha variants and mixed terms
        "((($x -> $y -> x y) (a)) (f)) ((($c -> $d -> c d) (a)) (f))",
        // Applications with mixed simple and nested alpha variants
        "(($x -> x y) (a)) (($c -> c y) (a)) ((($d -> d y) a) (f))",
        // More complex applications with a mix of repetition and alpha variants
        "((($x -> x y) (a)) (b)) ((($c -> c y) (a)) (b)) ((($f -> f y) (a)) (h))",
        // Nested with mixed alpha and non-repeating terms
        "((($x -> $y -> x y) (a)) (($b -> $c -> b c) (a) (e))) ((($f -> $g -> f g) (a)) (i) (j))",
        // Complex non-repeating applications
        "($x -> $y -> $z -> x (y (z w))) (a) (b) (c) (d)",
    ];
    let asts: Vec<AST> = exprs.iter().map(|expr| AST::eval(expr).unwrap()).collect();

    // Timing function_one
    let start = Instant::now();
    for _ in 0..ITERATIONS {
        eval_asts(asts.clone(), &EvalOrder::Normal)
    }
    let duration = start.elapsed();
    println!("Normal order reduction: {:?}", duration);

    // Timing function_two
    let start = Instant::now();
    for _ in 0..ITERATIONS {
        eval_asts(asts.clone(), &EvalOrder::Lazy)
    }
    let duration = start.elapsed();
    println!("Lazy reduction: {:?}", duration);

    // Timing function_three
    let start = Instant::now();
    for _ in 0..ITERATIONS {
        eval_asts(asts.clone(), &EvalOrder::Applicative)
    }
    let duration = start.elapsed();
    println!("Applicative reduction: {:?}", duration);
    // process_exprs(&exprs);
}
