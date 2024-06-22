use crate::alpha::variant::AlphaVariant;
use crate::ast::ast::AST;
use crate::ast::term::Term;
use crate::beta::memo::Memo;
use crate::substitution::substitution::Substitution;
pub trait BetaReduction {
    fn beta_step(self, memo: Memo) -> (AST, Memo);
    fn beta_reduction(self, memo: Option<Memo>) -> (AST, Memo);
    fn beta_reduce(self) -> AST;
}

impl BetaReduction for AST {
    fn beta_step(self, memo: Memo) -> (AST, Memo) {
        match self.is_reducible {
            false => (self, memo),
            true => match memo.get(self.clone()).term {
                Term::Var(s) => (AST::var(s), memo),
                Term::Abstr(param, body) => match (*body).beta_step(memo) {
                    (ast, new_memo) => (AST::abstr(*param, ast), new_memo),
                },
                Term::Apply(f, arg) => match (*f).term {
                    Term::Abstr(param, body) => {
                        let (ast, memo) = match body
                            .clone()
                            .substitute((*param).term.clone(), (*arg).clone())
                        {
                            Err(_) => (
                                (*body)
                                    .alpha_variant()
                                    .substitute((*param).term, *arg)
                                    .unwrap(),
                                memo,
                            ),
                            Ok(ast) => (ast, memo),
                        };
                        (ast.clone(), memo.with(self, ast))
                    }
                    _ => match f.beta_step(memo) {
                        (f_ast, new_memo) => match arg.beta_step(new_memo) {
                            (arg_ast, final_memo) => (AST::apply(f_ast, arg_ast), final_memo),
                        },
                    },
                },
            },
        }
    }
    fn beta_reduction(self, memo: Option<Memo>) -> (AST, Memo) {
        let memo = memo.unwrap_or(Memo::new());
        let (new_ast, new_memo) = self.clone().beta_step(memo);

        match !new_ast.is_reducible || new_memo.contains(&new_ast) {
            true => (new_ast, new_memo),
            false => new_ast
                .clone()
                .beta_reduction(Some(new_memo.with(self, new_ast))),
        }
    }

    fn beta_reduce(self) -> AST {
        let (result, memo) = self.beta_reduction(None);
        result
    }
}
