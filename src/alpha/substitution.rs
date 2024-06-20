use crate::alpha::renaming::Renaming;
use crate::ast::ast::{Term, Variable, Abstraction, Application, AST};


pub enum SubstitutionError {
    NotAVariable(String),
    RenamingError(String),
}
pub trait Substitution {
    fn substitute(self, var: Term, term: AST) -> Result<AST, SubstitutionError>;
}

impl Substitution for AST {
    fn substitute(self, var: Term, term: AST) -> Result<AST, SubstitutionError> {
        match &var {
            Term::Var(c) => match &*self {
                Term::Var(d) => match *c == *d {
                    true => Ok(term),
                    false => Ok(self),
                },
                Term::Apply(f, arg) => {
                    let lhs = (**f).substitute(var, term);
                    let rhs = (**arg).substitute(var, term);
                    match (lhs, rhs) {
                        (Err(e), _) | (_, Err(e)) => Err(e),
                        (Ok(new_f), Ok(new_arg)) => Ok(AST::apply(new_f, new_arg)),
                    }
                }
                Term::Abstr(param, body) =>
            },

            _ => Err(SubstitutionError::NotAVariable(format!(
                "Can only substitute variables, not {:?}",
                var
            ))),
        }
    }
}
