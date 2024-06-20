use crate::ast::new::ASTMaker;
use crate::ast::term::Term;
use crate::ast::varset::VarSet;
use std::fmt;
use std::ops::Deref;

#[derive(Hash, Eq, PartialEq, Clone)]
pub struct AST {
    pub term: Term,
    pub free_vars: VarSet,
    pub binding_vars: VarSet,
}

impl AST {
    pub fn all_vars(&self) -> VarSet {
        self.free_vars.clone() | self.binding_vars.clone()
    }
    pub fn fresh(varset: VarSet) -> AST {
        let mut candidates = ('a'..='z').chain('A'..='Z');
        let new_char = candidates
            .find(|&c| !varset.contains(&Term::Var(c)))
            .expect("Ran out of variable names");
        AST::var(new_char)
    }
}

impl Deref for AST {
    type Target = Term;

    fn deref(&self) -> &Self::Target {
        &self.term
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
