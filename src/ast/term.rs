use crate::ast::ast::AST;
use std::hash::Hash;

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub enum Term {
    Var(String),
    Abstr(Box<AST>, Box<AST>),
    Apply(Box<AST>, Box<AST>),
}
