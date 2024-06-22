use crate::ast::ast::AST;
use crate::ast::term::Term;

#[derive(Debug, Clone)]
pub enum SubstitutionError {
    NotAVariable(String),
    BindingConflict(String),
}
pub trait Substitution {
    fn substitute(self, var: Term, term: AST) -> Result<AST, SubstitutionError>;
}

impl Substitution for AST {
    fn substitute(self, var: Term, term: AST) -> Result<AST, SubstitutionError> {
        match &var {
            Term::Var(s) => {
                if !self.free_vars.contains(&var) {
                    return Ok(self);
                }
                if !(self.bound_vars() & term.free_vars.clone()).is_empty() {
                    return Err(SubstitutionError::BindingConflict(format!(
                        "Some of free variables of {:?} bound in {:?}",
                        term, self
                    )));
                }
                match self.term.clone() {
                    Term::Var(f) => match *s == f {
                        true => Ok(term),
                        false => Ok(self),
                    },
                    Term::Apply(f, arg) => {
                        let lhs = (*f).substitute(var.clone(), term.clone())?;
                        let rhs = (*arg).substitute(var, term)?;
                        Ok(AST::apply(lhs, rhs))
                    }
                    Term::Abstr(param, body) => match &term.free_vars.contains(&(*param).term) {
                        true => {
                            let fresh_var = AST::fresh(
                                (&*body).all_vars()
                                    | (&*param).free_vars.clone()
                                    | term.free_vars.clone(),
                            );
                            let renamed_body =
                                (*body).substitute((*param).term, fresh_var.clone())?;
                            let new_body = renamed_body.substitute(var, term)?;
                            Ok(AST::abstr(fresh_var, new_body))
                        }
                        false => {
                            let new_body = (*body).substitute(var, term)?;
                            Ok(AST::abstr(*param, new_body))
                        }
                    },
                }
            }

            _ => Err(SubstitutionError::NotAVariable(format!(
                "Can only substitute variables, not {:?}",
                var
            ))),
        }
    }
}
