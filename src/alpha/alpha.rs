use crate::alpha::renaming::Renaming;
use crate::ast::ast::{Term, Var, AST};
#[derive(Debug, Clone)]
pub enum Error {
    RenamingError(String),
    StructureError(String),
}
pub trait AlhaConvert {
    fn alpha_convert(&self, other: AST) -> Result<AST, Error>;
    fn convert_abstrs(lparam: &AST, lbody: &AST, rparam: AST, rbody: AST) -> Result<AST, Error>;
    fn convert_appls(lf: &AST, larg: &AST, rf: AST, rarg: AST) -> Result<AST, Error>;
}

impl AlhaConvert for AST {
    fn alpha_convert(&self, other: AST) -> Result<AST, Error> {
        if self.free_vars != other.free_vars {
            return Err(Error::StructureError(format!("Different free variables")));
        }
        match (&**self, *other) {
            (Term::Abstr(param, body), Term::Abstr(other_param, other_body)) => {
                if ***param == **other_param {
                    match (&**body).alpha_convert(other) {
                        Err(e) => Err(e),
                        Ok(new_body) => Ok(AST::abstr(*param.clone(), new_body)),
                    };
                }
                match (*other_body).rename(**other_param, (***param).clone()) {
                    Err(e) => Err(Error::RenamingError(format!("{:?}", e))),
                    Ok(new_body) => Ok(AST::abstr(*param.clone(), new_body)),
                }
            }
        }
    }
    fn convert_abstrs(lparam: &AST, lbody: &AST, rparam: AST, rbody: AST) -> Result<AST, Error> {}
    fn convert_appls(lf: &AST, larg: &AST, rf: AST, rarg: AST) -> Result<AST, Error> {}
}
