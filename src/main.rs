mod ast;
mod lexer;
mod parser;
mod substitution;
use ast::ast::AST;

use std::fs;
use std::io;

fn read_file(filename: &str) -> io::Result<String> {
    let content = fs::read_to_string(filename)?;
    Ok(content.chars().filter(|c| !c.is_whitespace()).collect())
}

fn process_pairs(pairs: &[(&str, &str)]) {
    for (a_str, b_str) in pairs {
        println!("{} <==> {}", a_str, b_str);
        match (AST::eval(a_str), AST::eval(b_str)) {
            (Ok(a), Ok(b)) => println!("{}", a == b),
            (Err(e), _) => println!("Error in first expression '{}': {:?}", a_str, e),
            (_, Err(e)) => println!("Error in second expression '{}': {:?}", b_str, e),
        }
    }
}

fn main() {
    let pairs = [
        // True
        ("$a -> a b", "$x -> x b"),
        ("$a -> $b -> a b c", "$x -> $y -> x y c"),
        ("$m -> $n -> m (n o)", "$p -> $q -> p (q o)"),
        ("$x -> $y -> x (y z)", "$a -> $b -> a (b z)"),
        ("$u -> $v -> $w -> u v w", "$a -> $b -> $c -> a b c"),
        ("($a -> a b) c", "($x -> x b) c"),
        ("$x -> ($y -> y x) z", "$a -> ($b -> b a) z"),
        ("$x -> $y -> ($z -> z x y) w", "$a -> $b -> ($c -> c a b) d"),
        ("$a -> $b -> ($c -> c) (a b)", "$x -> $y -> ($z -> z) (x y)"),
        ("$f -> $x -> f (x x) y", "$g -> $z -> g (z z) w"),
        ("$a -> $b -> a (b c d)", "$x -> $y -> x (y c d)"),
        (
            "$a -> $b -> $c -> a b (c d e)",
            "$x -> $y -> $z -> x y (z d e)",
        ),
        ("$xyz -> z y x", "$abc -> c b a"),
        // False
        ("$a -> a b", "$a -> b a"), // Different application order
        ("$a -> $b -> a b c", "$a -> $c -> a c b"), // Different variable names and order
        ("$m -> $n -> m (n o)", "$m -> $n -> n (m o)"), // Different application structure
        ("$x -> $y -> x (y z)", "$x -> $y -> y (x z)"), // Different application structure
        ("$u -> $v -> $w -> u v w", "$u -> $v -> $w -> u w v"), // Different application order
        ("($a -> a b) c", "($a -> a b) d"), // Different free variable in application
        ("$x -> ($y -> y x) z", "$x -> ($y -> y z) x"), // Different variable bindings and order
        ("$a -> $b -> a b", "$a -> $a -> a a"), // Variable name conflict
        ("$a -> a b", "$b -> b a"), // Free variable change
        ("$x -> $y -> x y z", "$x -> $y -> x y"), // Missing free variable
        ("$a -> $b -> a (b c)", "$a -> $b -> b (a c)"), // Different application order
        ("$a -> $b -> $c -> a (b c)", "$a -> $b -> $c -> c (a b)"), // Different application order
        ("$a -> a b", "$b -> b c"), // Different free variables
        ("$a -> ($b -> b) a", "$a -> ($b -> b) b"), // Free variable and binding conflict
        ("$a -> $b -> a (b c d)", "$x -> $y -> y (x c d)"), // Different application order
        ("$x -> $y -> x (y z)", "$a -> $b -> b (a z)"), // Different variable bindings
        ("$xyz -> x y z", "$abc -> a c b"), // Different application order
        ("$a -> $b -> ($c -> c) (a b)", "$x -> $y -> ($z -> z) (y x)"), // Different application order
    ];
    process_pairs(&pairs)
}
