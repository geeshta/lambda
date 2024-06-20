use crate::ast::ast::AST;
use crate::ast::term::Term;
use crate::ast::varset::VarSet;

pub trait ASTMaker {
    fn var(c: char) -> AST;
    fn abstr(param: AST, body: AST) -> AST;
    fn apply(f: AST, arg: AST) -> AST;
}

impl ASTMaker for AST {
    fn var(c: char) -> AST {
        let term = Term::Var(c);
        let free_vars = VarSet::from(term.clone());
        let binding_vars = VarSet::new();
        AST {
            term,
            free_vars,
            binding_vars,
        }
    }
    fn abstr(param: AST, body: AST) -> AST {
        let free_vars = body.free_vars.clone() - (*param).clone();
        let binding_vars = VarSet::from((*param).clone()) | body.binding_vars.clone();
        let term = Term::Abstr(Box::new(param), Box::new(body));
        AST {
            term,
            free_vars,
            binding_vars,
        }
    }
    fn apply(f: AST, arg: AST) -> AST {
        let free_vars = f.free_vars.clone() | arg.free_vars.clone();
        let binding_vars = f.binding_vars.clone() | arg.binding_vars.clone();
        let term = Term::Apply(Box::new(f), Box::new(arg));
        AST {
            term,
            free_vars,
            binding_vars,
        }
    }
}
