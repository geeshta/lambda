use crate::ast::freevars::FreeVars;
use std::fmt;
use std::ops::Deref;

pub trait Var {
    fn var(c: char) -> Self;
}

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub enum Term {
    Var(char),
    Abstr(Box<AST>, Box<AST>),
    Apply(Box<AST>, Box<AST>),
}

impl Var for Term {
    fn var(c: char) -> Self {
        Term::Var(c)
    }
}

#[derive(Hash, Eq, PartialEq, Clone)]
pub struct AST {
    term: Term,
    pub free_vars: FreeVars,
}

impl Deref for AST {
    type Target = Term;

    fn deref(&self) -> &Self::Target {
        &self.term
    }
}

impl AST {
    pub fn abstr(param: AST, body: AST) -> Self {
        let free_vars = body.free_vars.clone() - (*param).clone();
        let term = Term::Abstr(Box::new(param), Box::new(body));
        AST { term, free_vars }
    }

    pub fn apply(f: AST, arg: AST) -> Self {
        let free_vars = f.free_vars.clone() | arg.free_vars.clone();
        let term = Term::Apply(Box::new(f), Box::new(arg));
        AST { term, free_vars }
    }
}

impl Var for AST {
    fn var(c: char) -> Self {
        let term = Term::Var(c);
        let free_vars = FreeVars::from(term.clone());
        AST { term, free_vars }
    }
}

impl fmt::Debug for AST {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &**self {
            Term::Var(c) => write!(f, "{:?}", c),
            Term::Abstr(param, body) => write!(f, "Abst({:?}, {:?})", param, body),
            Term::Apply(func, arg) => write!(f, "Appl({:?}, {:?})", func, arg),
        }
    }
}
