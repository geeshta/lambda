use crate::ast::Term;
use crate::ast::AST;
use crate::variables::VarMap;

pub trait AlphaVariant {
    fn remap(self, map: VarMap) -> AST;
    fn alpha_variant(&self) -> AST;
}

impl AlphaVariant for AST {
    fn alpha_variant(&self) -> AST {
        let new_ast = self.clone();
        let fresh_set = new_ast.all_vars().fresh_set();
        let var_map = VarMap::from_sets(fresh_set, new_ast.binding_vars.clone());
        new_ast.remap(var_map)
    }

    fn remap(self, var_map: VarMap) -> AST {
        match self.term {
            Term::Var(_) => match var_map.get(self.term) {
                Term::Var(s) => AST::var(s),
                _ => panic!("Map of variables must only contain variables"),
            },
            Term::Abstr(param, body) => {
                AST::abstr(param.remap(var_map.clone()), body.remap(var_map))
            }
            Term::Apply(f, arg) => AST::apply(f.remap(var_map.clone()), arg.remap(var_map)),
        }
    }
}
