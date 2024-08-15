use crate::alpha::AlphaVariant;
use crate::ast::Term;
use crate::ast::TermMap;
use crate::ast::AST;
use crate::substitution::Substitution;

pub enum EvalOrder {
    Normal,
    Applicative,
    Lazy,
}
pub trait BetaReduction {
    fn reduce(self, order: Option<&EvalOrder>) -> AST;
}

impl AST {
    /// One step of a beta reduction. It is recursive so it may actually perform multiple substitutions
    fn beta_step(self, order: &EvalOrder) -> AST {
        match self.is_reducible {
            // If the term cannot be reduced further, just return it
            false => self,
            // Otherwise look up the current term in the memo
            true => match self.clone().term {
                // A variable reduces to itself. It could have come from the memo.
                Term::Var(s) => AST::var(s),
                // For abstraction, just recusively reduce the body
                Term::Abstr(param, body) => match (*body).beta_step(order) {
                    ast => AST::abstr(*param, ast),
                },
                // For application, it depends if it is a redex
                Term::Apply(f, arg) => match (*f).term {
                    // If it is a redex, perform the substitution of param in the body with the argument
                    Term::Abstr(param, body) => self.reduce_redex(order, *param, *body, *arg),
                    // If it is not a redex, just recursively reduce the left and right side
                    // The first reduction may enhance the memo so we need to call it one at a time
                    _ => {
                        let f_reduced = f.beta_step(order);
                        let arg_reduced = arg.beta_step(order);
                        AST::apply(f_reduced, arg_reduced)
                    }
                },
            },
        }
    }
    /// One step of a beta reduction. It is recursive so it may actually perform multiple substitutions
    fn lazy_beta_step(self, memo: TermMap) -> (AST, TermMap) {
        match self.is_reducible {
            // If the term cannot be reduced further, just return it
            false => (self, memo),
            // Otherwise look up the current term in the memo
            true => match memo.get(self.clone()).term {
                // A variable reduces to itself. It could have come from the memo.
                Term::Var(s) => (AST::var(s), memo),
                // For abstraction, just recusively reduce the body
                Term::Abstr(param, body) => match (*body).lazy_beta_step(memo) {
                    (ast, new_memo) => (AST::abstr(*param, ast), new_memo),
                },
                // For application, it depends if it is a redex
                Term::Apply(f, arg) => match (*f).term {
                    // If it is a redex, perform the substitution of param in the body with the argument
                    Term::Abstr(param, body) => self.lazy_reduce_redex(memo, *param, *body, *arg),
                    // If it is not a redex, just recursively reduce the left and right side
                    // The first reduction may enhance the memo so we need to call it one at a time
                    _ => match f.lazy_beta_step(memo) {
                        (f_ast, new_memo) => match arg.lazy_beta_step(new_memo) {
                            (arg_ast, final_memo) => (AST::apply(f_ast, arg_ast), final_memo),
                        },
                    },
                },
            },
        }
    }

    /// Reduction of a redex term, i.e. the application of an abstraction to another term
    fn reduce_redex(self, order: &EvalOrder, param: AST, body: AST, arg: AST) -> AST {
        // Try to perform the substitution as-is.
        match order {
            EvalOrder::Normal => body
                .clone()
                .substitute(param.term.clone(), arg.clone())
                // We expect that we may need to first create an alpha variant of the body if some of the
                // free variables of arg appear as bindning in body
                .unwrap_or_else(|_| body.alpha_variant().substitute(param.term, arg).unwrap()),
            EvalOrder::Applicative => body
                .clone()
                .substitute(param.term.clone(), arg.clone().reduce(Some(order)))
                // We expect that we may need to first create an alpha variant of the body if some of the
                // free variables of arg appear as bindning in body
                .unwrap_or_else(|_| {
                    body.alpha_variant()
                        .substitute(param.term, arg.reduce(Some(order)))
                        .unwrap()
                }),
            EvalOrder::Lazy => panic!("This function should never be called with lazy evaluation"),
        }
    }

    /// Reduction of a redex term, i.e. the application of an abstraction to another term
    fn lazy_reduce_redex(self, memo: TermMap, param: AST, body: AST, arg: AST) -> (AST, TermMap) {
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
    fn beta_reduce(self, order: &EvalOrder) -> AST {
        // Perform one beta step on the term
        let new_ast = self.clone().beta_step(order);
        // If the term is not reducible or has not yet been processed before, keep processiong
        match !new_ast.is_reducible {
            true => new_ast,
            // Before each new process, add the result of the previous call to the memo
            false => new_ast.clone().beta_reduce(order),
        }
    }

    /// The main reduction loop that may also never terminate
    fn lazy_beta_reduce(self, memo: Option<TermMap>) -> (AST, TermMap) {
        let memo = memo.unwrap_or(TermMap::new());
        // Perform one beta step on the term
        let (new_ast, new_memo) = self.clone().lazy_beta_step(memo);
        // If the term is not reducible or has not yet been processed before, keep processiong
        match !new_ast.is_reducible || new_memo.contains(&new_ast) {
            true => (new_ast, new_memo),
            // Before each new process, add the result of the previous call to the memo
            false => new_ast
                .clone()
                .lazy_beta_reduce(Some(new_memo.with(self, new_ast))),
        }
    }
}

impl BetaReduction for AST {
    /// A beta reduction loop that keeps the memo hidden
    fn reduce(self, order: Option<&EvalOrder>) -> AST {
        let order = order.unwrap_or(&EvalOrder::Normal);
        match order {
            EvalOrder::Lazy => match self.lazy_beta_reduce(None) {
                (result, memo) => result,
            },
            EvalOrder::Normal | EvalOrder::Applicative => self.beta_reduce(order),
        }
    }
}
