use crate::ast::ast::AST;
use std::hash::Hash;

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub enum Term {
    Var(char),
    Abstr(Box<AST>, Box<AST>),
    Apply(Box<AST>, Box<AST>),
}
