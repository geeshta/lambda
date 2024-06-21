use crate::alpha::alpha::AlhaConversion;
use crate::ast::term::Term;
use crate::ast::varset::VarSet;
use std::fmt;
use std::ops::Deref;

#[derive(Hash, Clone)]
pub struct AST {
    pub term: Term,
    pub free_vars: VarSet,
    pub binding_vars: VarSet,
}

impl AST {
    pub fn var(c: char) -> AST {
        let term = Term::Var(c);
        let free_vars = VarSet::from(term.clone());
        let binding_vars = VarSet::new();
        AST {
            term,
            free_vars,
            binding_vars,
        }
    }
    pub fn abstr(param: AST, body: AST) -> AST {
        let free_vars = body.free_vars.clone() - (*param).clone();
        let binding_vars = VarSet::from((*param).clone()) | body.binding_vars.clone();
        let term = Term::Abstr(Box::new(param), Box::new(body));
        AST {
            term,
            free_vars,
            binding_vars,
        }
    }
    pub fn apply(f: AST, arg: AST) -> AST {
        let free_vars = f.free_vars.clone() | arg.free_vars.clone();
        let binding_vars = f.binding_vars.clone() | arg.binding_vars.clone();
        let term = Term::Apply(Box::new(f), Box::new(arg));
        AST {
            term,
            free_vars,
            binding_vars,
        }
    }
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

impl PartialEq for AST {
    fn eq(&self, other: &Self) -> bool {
        match self.alpha_convert((*other).clone()) {
            Ok(_) => true,
            Err(_) => false,
        }
    }
}

impl Eq for AST {}

impl fmt::Debug for AST {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &**self {
            Term::Var(c) => write!(f, "{:?}", c),
            Term::Abstr(param, body) => write!(f, "Abst({:?}, {:?})", param, body),
            Term::Apply(func, arg) => write!(f, "Appl({:?}, {:?})", func, arg),
        }
    }
}
