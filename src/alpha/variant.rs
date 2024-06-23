use crate::ast::Term;
use crate::ast::AST;
use crate::variables::VarMap;

pub trait AlphaVariant {
    fn remap(self, map: VarMap) -> AST;
    fn alpha_variant(&self) -> AST;
}

impl AlphaVariant for AST {
    // Remap all varaibles in a term for different ones
    fn remap(self, var_map: VarMap) -> AST {
        match self.term {
            // Remap a variable - try to find a corresponding varaible in the map
            Term::Var(_) => match var_map.get(self.term) {
                Term::Var(s) => AST::var(s),
                _ => panic!("Map of variables must only contain variables"),
            },
            // Remap an abstraction - remap the param and body
            Term::Abstr(param, body) => {
                AST::abstr(param.remap(var_map.clone()), body.remap(var_map))
            }
            // Remap an application - remap the left and right terms
            Term::Apply(f, arg) => AST::apply(f.remap(var_map.clone()), arg.remap(var_map)),
        }
    }
    // Create an alpha variant of a term
    fn alpha_variant(&self) -> AST {
        let new_ast = self.clone();
        // Generate a set of fresh variables that are not present in the term at all
        let fresh_set = new_ast.all_vars().fresh_set();
        // Map them onto all binding variables in the term
        let var_map = VarMap::from_sets(fresh_set, new_ast.binding_vars.clone());
        // Perform the remapping
        new_ast.remap(var_map)
    }
}
