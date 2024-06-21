use crate::alpha::substitution::Substitution;
use crate::ast::ast::AST;
use crate::ast::term::Term;
#[derive(Debug, Clone)]
pub enum AlphaConvError {
    SubstitutionError(String),
    StructureError(String),
    BindingError(String),
    VariablesError(String),
}
pub trait AlhaConversion {
    fn alpha_convert(&self, other: AST) -> Result<AST, AlphaConvError>;
}

impl AlhaConversion for AST {
    fn alpha_convert(&self, other: AST) -> Result<AST, AlphaConvError> {
        if self.free_vars != other.free_vars {
            return Err(AlphaConvError::VariablesError(format!(
                "Different free variables: {:?} and {:?} in {:?} and {:?}",
                &self.free_vars, other.free_vars, &self, other
            )));
        }
        match (&**self, other.term) {
            (Term::Abstr(param, body), Term::Abstr(other_param, other_body)) => {
                if (&**body).binding_vars.contains(&***param)
                    != (&*other_body).binding_vars.contains(&**other_param)
                {
                    return Err((AlphaConvError::BindingError(format!(
                        "One of {:?} and {:?} is binding and other is not in {:?} {:?}",
                        ***param, **other_param, **body, *other_body
                    ))));
                }
                match (*other_body).substitute((*other_param).term, (**param).clone()) {
                    Err(e) => Err(AlphaConvError::SubstitutionError(format!("{:?}", e))),
                    Ok(renamed_body) => match (&**body).alpha_convert(renamed_body) {
                        Err(e) => Err(e),
                        Ok(converted_body) => Ok(AST::abstr((**param).clone(), converted_body)),
                    },
                }
            }
            (Term::Apply(f, arg), Term::Apply(other_f, other_arg)) => {
                let lhs = (&**f).alpha_convert(*other_f);
                let rhs = (&**arg).alpha_convert(*other_arg);
                match (lhs, rhs) {
                    (Err(e), _) | (_, Err(e)) => Err(e),
                    (Ok(new_f), Ok(new_arg)) => Ok(AST::apply(new_f, new_arg)),
                }
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
