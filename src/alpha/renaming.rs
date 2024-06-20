use crate::ast::ast::{Abstraction, Term, Var, AST};

#[derive(Debug, Clone)]
pub enum RenamingError {
    RenamingError(String),
    Conflict(String),
}

pub trait Renaming {
    fn rename(self, old_var: Term, new_var: Term) -> Result<AST, RenamingError>;
}

impl<Abstraction> Renaming for AST<Abstraction> {
    fn rename(self, old_var: Term, new_var: Term) -> Result<AST, RenamingError> {
        match (&old_var, &new_var) {
            (Term::Var(x), Term::Var(y)) => match self.term {
                Term::Var(z) => match z == *x {
                    true => Ok(AST::var(*y)),
                    false => Ok(self),
                },
                Term::Apply(f, arg) => {
                    let new_f = match self.free_vars.contains(&old_var) {
                        true => f.rename(old_var.clone(), new_var.clone()),
                        false => Ok(*f),
                    };
                    let new_arg = match self.free_vars.contains(&old_var) {
                        true => arg.rename(old_var.clone(), new_var.clone()),
                        false => Ok(*arg),
                    };
                    match (&new_f, &new_arg) {
                        (Err(e), _) | (_, Err(e)) => Err((*e).clone()),
                        _ => Ok(AST::apply(new_f.unwrap(), new_arg.unwrap())),
                    }
                }
                Term::Abstr(param, body) => {
                    if **param == new_var {
                        return Err(RenamingError::Conflict(format!(
                            "Variable {:?} is bound in lambda",
                            new_var
                        )));
                    }
                    if (*body).free_vars.contains(&new_var) {
                        return Err(RenamingError::Conflict(format!(
                            "Variable {:?} is free in lambda body",
                            new_var
                        )));
                    }
                    let new_body = match (*body).free_vars.contains(&old_var) {
                        true => (*body).rename(old_var, new_var),
                        false => Ok(*body),
                    };
                    match new_body {
                        Err(e) => Err(e),
                        Ok(new_ast) => Ok(AST::abstr(*param, new_ast)),
                    }
                }
            },
            _ => Err(RenamingError::RenamingError(format!(
                "Can only rename variables, found {:?} and {:?}",
                old_var, new_var
            ))),
        }
    }
}
