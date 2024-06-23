use crate::ast::Term;
use crate::ast::AST;

#[derive(Debug, Clone)]
pub enum SubstitutionError {
    NotAVariable(String),
    BindingConflict(String),
}
pub trait Substitution {
    fn substitute(self, var: Term, term: AST) -> Result<AST, SubstitutionError>;
    fn check_substitution<F>(
        &self,
        var: &Term,
        term: &AST,
        substitute_fn: F,
    ) -> Result<AST, SubstitutionError>
    where
        F: FnOnce(AST, Term, AST) -> Result<AST, SubstitutionError>;
}

impl Substitution for AST {
    /// Helper function that handles all the validation and takes in the main substitution as an argument
    fn check_substitution<F>(
        &self,
        var: &Term,
        term: &AST,
        substitute_fn: F,
    ) -> Result<AST, SubstitutionError>
    where
        F: FnOnce(AST, Term, AST) -> Result<AST, SubstitutionError>,
    {
        match var {
            // Var has to actually be a variable
            Term::Var(_) => {
                // If there's nothing to substitute, return
                if !self.free_vars.contains(var) {
                    return Ok(self.clone());
                }
                // A free variable in term would become bound if it is binding in self
                if !(self.bound_vars() & term.free_vars.clone()).is_empty() {
                    return Err(SubstitutionError::BindingConflict(format!(
                        "Some of free variables of {:?} bound in {:?}",
                        term, self
                    )));
                }
                // Call the main substitution function
                substitute_fn(self.clone(), var.clone(), term.clone())
            }
            _ => Err(SubstitutionError::NotAVariable(format!(
                "Can only substitute variables, not {:?}",
                var
            ))),
        }
    }

    fn substitute(self, var: Term, term: AST) -> Result<AST, SubstitutionError> {
        self.check_substitution(&var, &term, |ast, var, term| match ast.term.clone() {
            // Variable - substitute if it is equal to var
            Term::Var(f) => match var {
                Term::Var(ref s) if s == &f => Ok(term),
                _ => Ok(ast),
            },
            // Application - recursively substitute left and right sides
            Term::Apply(f, arg) => {
                let lhs = (*f).substitute(var.clone(), term.clone())?;
                let rhs = (*arg).substitute(var, term)?;
                Ok(AST::apply(lhs, rhs))
            }
            // Abstraction - first check if the current parameter is free somewhere in
            // the term we are replacing with
            Term::Abstr(param, body) => match term.free_vars.contains(&(*param).term) {
                // If so, we need to rename the parameter and all of its corresponding free
                // occurrences in body
                true => {
                    let fresh_var = AST::fresh(
                        (&*body).all_vars() | (*param).free_vars.clone() | term.free_vars.clone(),
                    );
                    let renamed_body =
                        (*body).substitute((*param).term.clone(), fresh_var.clone())?;
                    // After renaming, substitute in the body
                    let new_body = renamed_body.substitute(var, term)?;
                    Ok(AST::abstr(fresh_var, new_body))
                }
                // Otherwise just substitute in the body
                false => {
                    let new_body = (*body).substitute(var, term)?;
                    Ok(AST::abstr(*param, new_body))
                }
            },
        })
    }
}
