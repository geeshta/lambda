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
        match &var {
            Term::Var(c) => match self.term {
                Term::Var(d) => match *c == d {
                    true => Ok(term),
                    false => Ok(self),
                },
                Term::Apply(f, arg) => {
                    let lhs = (*f).substitute(var.clone(), term.clone());
                    let rhs = (*arg).substitute(var, term);
                    match (lhs, rhs) {
                        (Err(e), _) | (_, Err(e)) => Err(e),
                        (Ok(new_f), Ok(new_arg)) => Ok(AST::apply(new_f, new_arg)),
                    }
                }
                Term::Abstr(param, body) => match &term.free_vars.contains(&**param) {
                    true => {
                        let fresh_var =
                            AST::fresh((&*body).all_vars() | (&*param).free_vars.clone());
                        match (*body).substitute((*param).term, fresh_var.clone()) {
                            Err(e) => Err(e),
                            Ok(renamed_body) => match renamed_body.substitute(var, term) {
                                Err(e) => Err(e),
                                Ok(new_body) => Ok(AST::abstr(fresh_var, new_body)),
                            },
                        }
                    }
                    false => match (*body).substitute(var, term) {
                        Err(e) => Err(e),
                        Ok(new_body) => Ok(AST::abstr(*param, new_body)),
                    },
                },
            },

            _ => Err(SubstitutionError::NotAVariable(format!(
                "Can only substitute variables, not {:?}",
                var
            ))),
        }
    }
}
