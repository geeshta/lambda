use crate::ast::ast::AST;
use crate::ast::term::Term;

#[derive(Debug, Clone)]
pub enum SubstitutionError {
    NotAVariable(String),
}
pub trait Substitution {
    fn substitute(self, var: Term, term: AST) -> Result<AST, SubstitutionError>;
}

impl Substitution for AST {
    fn substitute(self, var: Term, term: AST) -> Result<AST, SubstitutionError> {
        if self.free_vars.contains(&var) {
            return Ok(self);
        }
        match &var {
            Term::Var(c) => match self.term {
                Term::Var(d) => match *c == d {
                    true => Ok(term),
                    false => Ok(self),
                },
                Term::Apply(f, arg) => {
                    let lhs = (*f).substitute(var.clone(), term.clone())?;
                    let rhs = (*arg).substitute(var, term)?;
                    Ok(AST::apply(lhs, rhs))
                }
                Term::Abstr(param, body) => match &term.free_vars.contains(&**param) {
                    true => {
                        let fresh_var =
                            AST::fresh((&*body).all_vars() | (&*param).free_vars.clone());
                        let renamed_body = (*body).substitute((*param).term, fresh_var.clone())?;
                        let new_body = renamed_body.substitute(var, term)?;
                        Ok(AST::abstr(fresh_var, new_body))
                    }
                    false => {
                        let new_body = (*body).substitute(var, term)?;
                        Ok(AST::abstr(*param, new_body))
                    }
                },
            },

            _ => Err(SubstitutionError::NotAVariable(format!(
                "Can only substitute variables, not {:?}",
                var
            ))),
        }
    }
}
