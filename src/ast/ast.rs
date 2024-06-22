use crate::alpha::AlhaConversion;
use crate::ast::Term;
use crate::lexer::tokenize;
use crate::parser::{parse, ParsingError};
use crate::variables::VarGen;
use crate::variables::VarSet;
use regex::Error as RegexError;
use std::cmp::Ordering;
use std::fmt;

#[derive(Debug)]
pub enum EvalError {
    TokenizationError(String),
    ParsingError(String),
}

impl From<RegexError> for EvalError {
    fn from(err: RegexError) -> Self {
        EvalError::TokenizationError(format!("{:?}", err))
    }
}

impl From<ParsingError> for EvalError {
    fn from(err: ParsingError) -> Self {
        EvalError::ParsingError(format!("{:?}", err))
    }
}

#[derive(Hash, Clone)]
pub struct AST {
    pub term: Term,
    pub free_vars: VarSet,
    pub binding_vars: VarSet,
    pub is_reducible: bool,
}

impl AST {
    pub fn var(s: String) -> AST {
        let term = Term::Var(s);
        let free_vars = VarSet::from(term.clone());
        let binding_vars = VarSet::new();
        AST {
            term,
            free_vars,
            binding_vars,
            is_reducible: false,
        }
    }
    pub fn abstr(param: AST, body: AST) -> AST {
        let free_vars = body.free_vars.clone() - param.term.clone();
        let binding_vars = VarSet::from(param.term.clone()) | body.binding_vars.clone();
        let term = Term::Abstr(Box::new(param), Box::new(body.clone()));
        AST {
            term,
            free_vars,
            binding_vars,
            is_reducible: body.is_reducible,
        }
    }
    pub fn apply(f: AST, arg: AST) -> AST {
        let free_vars = f.free_vars.clone() | arg.free_vars.clone();
        let binding_vars = f.binding_vars.clone() | arg.binding_vars.clone();
        let term = Term::Apply(Box::new(f.clone()), Box::new(arg.clone()));
        let is_reducible = match f.term {
            Term::Abstr(_, _) => true,
            _ => f.is_reducible || arg.is_reducible,
        };
        AST {
            term,
            free_vars,
            binding_vars,
            is_reducible,
        }
    }
    pub fn all_vars(&self) -> VarSet {
        self.free_vars.clone() | self.binding_vars.clone()
    }

    pub fn bound_vars(&self) -> VarSet {
        self.free_vars.clone() & self.binding_vars.clone()
    }

    pub fn fresh(varset: VarSet) -> AST {
        let mut candidates = VarGen::new();
        let new_var = candidates
            .find(|name| !varset.contains(&Term::Var(name.clone())))
            .expect("Ran out of variable names");
        println!("Generated new name: {:?}", new_var);
        AST::var(new_var)
    }

    pub fn eval(input: &str) -> Result<AST, EvalError> {
        let tokens = tokenize(input)?;
        let ast = parse(&tokens)?;
        Ok(ast)
    }
}

impl PartialEq for AST {
    fn eq(&self, other: &Self) -> bool {
        match self.free_vars == other.free_vars {
            false => false,
            true => {
                let right = &other.alpha_convert(self.clone());
                let left = self.alpha_convert(other.clone());
                match (left, right) {
                    (Err(_), _) | (_, Err(_)) => false,
                    _ => true,
                }
            }
        }
    }
}

impl Eq for AST {}

impl Ord for AST {
    fn cmp(&self, other: &AST) -> Ordering {
        match self == other {
            true => Ordering::Equal,
            _ => format!("{:?}", self).cmp(&format!("{:?}", other)),
        }
    }
}

impl PartialOrd for AST {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Debug for AST {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &(*self).term {
            Term::Var(s) => write!(f, "{:?}", s),
            Term::Abstr(param, body) => write!(f, "(${:?} -> {:?})", param, body),
            Term::Apply(func, arg) => write!(f, "{:?} {:?}", func, arg),
        }
    }
}
