use crate::ast::ast::{Term, Var, AST};

#[derive(Debug)]
pub enum Error {
    RenamingError(String),
    Conflict(String),
}

pub trait Renaming {
    fn rename(self, old_var: Term, new_var: Term) -> Result<AST, Error>;
}

impl Renaming for AST {
    fn rename(self, old_var: Term, new_var: Term) -> Result<AST, Error> {
        match (&old_var, &new_var) {
            (Term::Var(c), Term::Var(d)) => match &*self {
                Term::Abstr(param, body) => {
                    if self.free_vars.contains(&new_var) || new_var == ***param {
                        return Err(Error::Conflict(format!(
                            "{:?} is either bound or free in the body of {:?}",
                            new_var, self
                        )));
                    }
                    let body_expr = body.clone();
                    match body_expr.rename(old_var, new_var.clone()) {
                        Err(e) => Err(e),
                        Ok(expr) => Ok(AST::abstr(AST::var(*d), expr)),
                    }
                }
                Term::Apply(f, arg) => {
                    let new_f = match f.free_vars.contains(&old_var) {
                        true => (f.clone())
                            .rename(old_var.clone(), new_var.clone())
                            .map(Box::new),
                        false => Ok(f.clone()),
                    };
                    let new_arg = match arg.free_vars.contains(&old_var) {
                        true => (arg.clone()).rename(old_var, new_var).map(Box::new),
                        false => Ok(arg.clone()),
                    };
                    match (new_f, new_arg) {
                        (Ok(new_f_expr), Ok(new_arg_expr)) => {
                            Ok(AST::apply(*new_f_expr, *new_arg_expr))
                        }
                        (Err(e), _) | (_, Err(e)) => Err(e),
                    }
                }
                Term::Var(a) => match a == c {
                    false => Ok(self),
                    true => Ok(AST::var(*d)),
                },
            },
            _ => Err(Error::RenamingError(format!(
                "Can only rename variables, found {:?} and {:?}",
                old_var, new_var
            ))),
        }
    }
}
