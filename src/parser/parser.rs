use crate::ast::ast::AST;
use crate::ast::term::Term;
use crate::lexer::lexer::Token;

#[derive(Debug)]
pub enum ParsingError {
    MismatchedParens(String),
    InvalidSyntax(String),
    InvalidLambda(String),
}

#[derive(Debug)]
pub enum ParseResult<'a> {
    Continue(AST, &'a [Token]),
    Stop(AST, &'a [Token]),
    End(&'a [Token]),
    Error(ParsingError),
}

fn parse_body(tokens: &[Token]) -> ParseResult {
    match tokens {
        [Token::Var(c), rest @ ..] => ParseResult::Continue(AST::var(*c), rest),
        [Token::LParen, rest @ ..] => group(rest),
        [Token::Lambda, rest @ ..] => lambda(rest),
        [Token::RParen, rest @ ..] => ParseResult::End(rest),
        [] => ParseResult::End(&[]),
        _ => ParseResult::Error(ParsingError::InvalidSyntax(format!(
            "Unexpected token in expression body: {:?}",
            tokens[0]
        ))),
    }
}

fn parse_params(tokens: &[Token]) -> ParseResult {
    match tokens {
        [Token::Var(c), rest @ ..] => ParseResult::Continue(AST::var(*c), rest),
        [Token::Arrow, rest @ ..] => ParseResult::End(rest),
        [] => ParseResult::End(&[]),
        _ => ParseResult::Error(ParsingError::InvalidSyntax(format!(
            "Unexpected token in lambda params: {:?}",
            tokens[0]
        ))),
    }
}

fn apply(term: AST, tokens: &[Token]) -> ParseResult {
    match parse_body(tokens) {
        ParseResult::Error(e) => ParseResult::Error(e),
        ParseResult::End(rest) => ParseResult::Stop(term, rest),
        ParseResult::Stop(new_expr, rest) => ParseResult::Stop(AST::apply(term, new_expr), rest),
        ParseResult::Continue(new_expr, rest) => match apply(AST::apply(term, new_expr), rest) {
            ParseResult::Error(e) => ParseResult::Error(e),
            ParseResult::End(_) => {
                panic!("Recursive call to 'apply' should never return an 'End' result")
            }
            ParseResult::Stop(last_expr, last_rest) => ParseResult::Stop(last_expr, last_rest),
            ParseResult::Continue(last_expr, last_rest) => apply(last_expr, last_rest),
        },
    }
}

fn abstr(arg: AST, tokens: &[Token]) -> ParseResult {
    match parse_body(tokens) {
        ParseResult::Error(e) => ParseResult::Error(e),
        ParseResult::End(_) => ParseResult::Error(ParsingError::MismatchedParens(format!(
            "Unexpected ')' at the start of a lambda expression after {:?}",
            arg
        ))),
        ParseResult::Stop(new_expr, rest) => ParseResult::Stop(AST::abstr(arg, new_expr), rest),
        ParseResult::Continue(new_expr, rest) => match apply(new_expr, rest) {
            ParseResult::Error(e) => ParseResult::Error(e),
            ParseResult::End(_) => {
                panic!("Recursive call to 'apply' should never return an 'End' result")
            }
            ParseResult::Continue(_, _) => {
                panic!("Recursive call to 'apply' should never return a 'Continue' result",)
            }
            ParseResult::Stop(last_expr, last_rest) => {
                ParseResult::Stop(AST::abstr(arg, last_expr), last_rest)
            }
        },
    }
}

fn lambda(tokens: &[Token]) -> ParseResult {
    match parse_params(tokens) {
        ParseResult::Error(e) => ParseResult::Error(e),
        ParseResult::End(rest) => ParseResult::End(rest),
        ParseResult::Stop(_, _) => {
            panic!("A call to 'parse_params' should never return a 'Stop' result")
        }
        ParseResult::Continue(expr, rest) => match *expr {
            Term::Var(_) => match lambda(rest) {
                ParseResult::Error(e) => ParseResult::Error(e),
                ParseResult::End(new_rest) => abstr(expr, new_rest),
                ParseResult::Stop(new_expr, new_rest) => {
                    ParseResult::Continue(AST::abstr(expr, new_expr), new_rest)
                }
                ParseResult::Continue(new_expr, new_rest) => {
                    ParseResult::Continue(AST::abstr(expr, new_expr), new_rest)
                }
            },
            _ => ParseResult::Error(ParsingError::InvalidLambda(format!(
                "Lambda expression arguments may only contain variables, found {:?}",
                expr
            ))),
        },
    }
}

fn group(tokens: &[Token]) -> ParseResult {
    match parse_body(tokens) {
        ParseResult::Error(e) => ParseResult::Error(e),
        ParseResult::End(_) => ParseResult::Error(ParsingError::MismatchedParens(
            "Empty parentheses '()' are not permitted".to_string(),
        )),
        ParseResult::Stop(expr, rest) => ParseResult::Continue(expr, rest),
        ParseResult::Continue(expr, rest) => match apply(expr, rest) {
            ParseResult::Error(e) => ParseResult::Error(e),
            ParseResult::End(_) => {
                panic!("Recursive call to 'apply' should never return an 'End' result")
            }
            ParseResult::Continue(_, _) => {
                panic!("Recursive call to 'apply' should never return a 'Continue' result")
            }
            ParseResult::Stop(last_expr, last_rest) => ParseResult::Continue(last_expr, last_rest),
        },
    }
}

pub fn parse(tokens: &[Token]) -> Result<AST, ParsingError> {
    match parse_body(tokens) {
        ParseResult::Error(e) => Err(e),
        ParseResult::End(_) => Err(ParsingError::MismatchedParens(format!(
            "Unexpected ')' at the start of expression"
        ))),
        ParseResult::Stop(expr, _) => Ok(expr),
        ParseResult::Continue(new_expr, rest) => match apply(new_expr, rest) {
            ParseResult::Error(e) => Err(e),
            ParseResult::End(_) => {
                panic!("Recursive call to 'apply' should never return an 'End' result")
            }
            ParseResult::Continue(_, _) => {
                panic!("Recursive call to 'apply' should never return a 'Continue' result")
            }
            ParseResult::Stop(last_expr, _) => Ok(last_expr),
        },
    }
}
