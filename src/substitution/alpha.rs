use crate::ast::ast::AST;
use crate::ast::term::Term;
use crate::substitution::substitution::{Substitution, SubstitutionError};
#[derive(Debug, Clone)]
pub enum AlphaConvError {
    SubstitutionError(String),
    StructureError(String),
    VariablesError(String),
}

impl From<SubstitutionError> for AlphaConvError {
    fn from(err: SubstitutionError) -> Self {
        AlphaConvError::SubstitutionError(format!("{:?}", err))
    }
}
pub trait AlhaConversion {
    fn alpha_convert(&self, other: AST) -> Result<AST, AlphaConvError>;
}

impl AlhaConversion for AST {
    fn alpha_convert(&self, other: AST) -> Result<AST, AlphaConvError> {
        match (&**self, other.term) {
            (Term::Abstr(param, body), Term::Abstr(other_param, other_body)) => {
                let renamed_body =
                    (*other_body).substitute((*other_param).term, (**param).clone())?;
                let converted_body = (&**body).alpha_convert(renamed_body)?;
                Ok(AST::abstr((**param).clone(), converted_body))
            }
            (Term::Apply(f, arg), Term::Apply(other_f, other_arg)) => {
                let lhs = (&**f).alpha_convert(*other_f)?;
                let rhs = (&**arg).alpha_convert(*other_arg)?;
                Ok(AST::apply(lhs, rhs))
            }
            (Term::Var(c), Term::Var(d)) => match *c == d {
                true => Ok(AST::var(*c)),
                false => Err(AlphaConvError::VariablesError(format!(
                    "Free variables {:?} and {:?} do not match",
                    *c, d,
                ))),
            },
            (a, b) => Err(AlphaConvError::StructureError(format!(
                "Different subterms: {:?} and {:?}",
                &a, b
            ))),
        }
    }
}
