use crate::ast::Term;
use crate::ast::AST;
use crate::substitution::{Substitution, SubstitutionError};

/// Type that represents and error during alpha conversions
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
    /// Tries to convert the other term into an alpha variant of self, if possible
    fn alpha_convert(&self, other: AST) -> Result<AST, AlphaConvError> {
        match (&(*self).term, other.term) {
            // Conversion of variables
            (Term::Var(s), Term::Var(d)) => match *s == d {
                // If remaining free variables are different, the conversion failed
                true => Ok(AST::var(s.clone())),
                false => Err(AlphaConvError::VariablesError(format!(
                    "Free variables {:?} and {:?} do not match",
                    *s, d,
                ))),
            },
            // Conversion of an abstraction
            (Term::Abstr(param, body), Term::Abstr(other_param, other_body)) => {
                // First substitute all occurences of the other term's parameter with this one's
                let renamed_body =
                    (*other_body).substitute((*other_param).term, (**param).clone())?;
                // Then recursively alpha convert the other body to this body
                let converted_body = (&**body).alpha_convert(renamed_body)?;
                // Return a new lambda with the parameter of this term and the converted body
                Ok(AST::abstr((**param).clone(), converted_body))
            }
            // Conversion of an application
            (Term::Apply(f, arg), Term::Apply(other_f, other_arg)) => {
                // Just recursively convert the left and right terms
                let lhs = (&**f).alpha_convert(*other_f)?;
                let rhs = (&**arg).alpha_convert(*other_arg)?;
                Ok(AST::apply(lhs, rhs))
            }
            // Cannot convert different terms
            (a, b) => Err(AlphaConvError::StructureError(format!(
                "Different subterms: {:?} and {:?}",
                &a, b
            ))),
        }
    }
}
