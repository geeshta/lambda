use crate::alpha::AlphaVariant;
use crate::ast::Term;
use crate::ast::AST;
use crate::beta::Memo;
use crate::substitution::Substitution;
pub trait BetaReduction {
    fn beta_step(self, memo: Memo) -> (AST, Memo);
    fn beta_reduce(self, memo: Option<Memo>) -> (AST, Memo);
    fn reduce(self) -> AST;
    fn reduce_redex(self, memo: Memo, param: AST, body: AST, arg: AST) -> (AST, Memo);
}

impl BetaReduction for AST {
    /// One step of a beta reduction. It is recursive so it may actually perform multiple substitutions
    fn beta_step(self, memo: Memo) -> (AST, Memo) {
        match self.is_reducible {
            // If the term cannot be reduced further, just return it
            false => (self, memo),
            // Otherwise look up the current term in the memo
            true => match memo.get(self.clone()).term {
                // A variable reduces to itself. It could have come from the memo.
                Term::Var(s) => (AST::var(s), memo),
                // For abstraction, just recusively reduce the body
                Term::Abstr(param, body) => match (*body).beta_step(memo) {
                    (ast, new_memo) => (AST::abstr(*param, ast), new_memo),
                },
                // For application, it depends if it is a redex
                Term::Apply(f, arg) => match (*f).term {
                    // If it is a redex, perform the substitution of param in the body with the argument
                    Term::Abstr(param, body) => self.reduce_redex(memo, *param, *body, *arg),
                    // If it is not a redex, just recursively reduce the left and right side
                    // The first reduction may enhance the memo so we need to call it one at a time
                    _ => match f.beta_step(memo) {
                        (f_ast, new_memo) => match arg.beta_step(new_memo) {
                            (arg_ast, final_memo) => (AST::apply(f_ast, arg_ast), final_memo),
                        },
                    },
                },
            },
        }
    }

    /// Reduction of a redex term, i.e. the application of an abstraction to another term
    fn reduce_redex(self, memo: Memo, param: AST, body: AST, arg: AST) -> (AST, Memo) {
        // Try to perform the substitution as-is.
        let substituted_body = body
            .clone()
            .substitute(param.term.clone(), arg.clone())
            // We expect that we may need to first create an alpha variant of the body if some of the
            // free variables of arg appear as bindning in body
            .unwrap_or_else(|_| body.alpha_variant().substitute(param.term, arg).unwrap());
        // Add the pair pair of evaluation to the memo for later lazy evaluation
        (substituted_body.clone(), memo.with(self, substituted_body))
    }

    /// The main reduction loop that may also never terminate
    fn beta_reduce(self, memo: Option<Memo>) -> (AST, Memo) {
        let memo = memo.unwrap_or(Memo::new());
        // Perform one beta step on the term
        let (new_ast, new_memo) = self.clone().beta_step(memo);
        // If the term is not reducible or has not yet been processed before, keep processiong
        match !new_ast.is_reducible || new_memo.contains(&new_ast) {
            true => (new_ast, new_memo),
            // Before each new process, add the result of the previous call to the memo
            false => new_ast
                .clone()
                .beta_reduce(Some(new_memo.with(self, new_ast))),
        }
    }

    /// A beta reduction loop that keeps the memo hidden
    fn reduce(self) -> AST {
        let (result, memo) = self.beta_reduce(None);
        result
    }
}
