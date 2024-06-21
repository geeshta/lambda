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
pub enum ParserState<'a> {
    Continue(AST, &'a [Token]),
    Stop(AST, &'a [Token]),
    End(&'a [Token]),
}

fn parse_body(tokens: &[Token]) -> Result<ParserState, ParsingError> {
    match tokens {
        [Token::Var(c), rest @ ..] => Ok(ParserState::Continue(AST::var(*c), rest)),
        [Token::LParen, rest @ ..] => group(rest),
        [Token::Lambda, rest @ ..] => lambda(rest),
        [Token::RParen, rest @ ..] => Ok(ParserState::End(rest)),
        [] => Ok(ParserState::End(&[])),
        _ => Err(ParsingError::InvalidSyntax(format!(
            "Unexpected token in expression body: {:?}",
            tokens[0]
        ))),
    }
}

fn parse_params(tokens: &[Token]) -> Result<ParserState, ParsingError> {
    match tokens {
        [Token::Var(c), rest @ ..] => Ok(ParserState::Continue(AST::var(*c), rest)),
        [Token::Arrow, rest @ ..] => Ok(ParserState::End(rest)),
        [] => Err(ParsingError::InvalidLambda(format!(
            "Unexpected end of file when reading lambda params"
        ))),
        _ => Err(ParsingError::InvalidSyntax(format!(
            "Unexpected token in lambda params: {:?}",
            tokens[0]
        ))),
    }
}

fn apply(term: AST, tokens: &[Token]) -> Result<ParserState, ParsingError> {
    let result = parse_body(tokens)?;
    match result {
        ParserState::End(rest) => Ok(ParserState::Stop(term, rest)),
        ParserState::Stop(new_expr, rest) => {
            Ok(ParserState::Stop(AST::apply(term, new_expr), rest))
        }
        ParserState::Continue(new_expr, rest) => {
            let new_result = apply(AST::apply(term, new_expr), rest)?;
            match new_result {
                ParserState::End(_) => {
                    panic!("Recursive call to 'apply' should never return an 'End' result")
                }
                ParserState::Stop(last_expr, last_rest) => {
                    Ok(ParserState::Stop(last_expr, last_rest))
                }
                ParserState::Continue(last_expr, last_rest) => apply(last_expr, last_rest),
            }
        }
    }
}

fn abstr(arg: AST, tokens: &[Token]) -> Result<ParserState, ParsingError> {
    let result = parse_body(tokens)?;
    match result {
        ParserState::End(_) => Err(ParsingError::MismatchedParens(format!(
            "Unexpected ')' at the start of a lambda expression after {:?}",
            arg
        ))),
        ParserState::Stop(new_expr, rest) => Ok(ParserState::Stop(AST::abstr(arg, new_expr), rest)),
        ParserState::Continue(new_expr, rest) => {
            let new_result = apply(new_expr, rest)?;
            match new_result {
                ParserState::End(_) => {
                    panic!("Recursive call to 'apply' should never return an 'End' result")
                }
                ParserState::Stop(last_expr, last_rest) => {
                    Ok(ParserState::Stop(AST::abstr(arg, last_expr), last_rest))
                }
                ParserState::Continue(_, _) => {
                    panic!("Recursive call to 'apply' should never return a 'Continue' result",)
                }
            }
        }
    }
}

fn lambda(tokens: &[Token]) -> Result<ParserState, ParsingError> {
    let result = parse_params(tokens)?;
    match result {
        ParserState::End(rest) => Ok(ParserState::End(rest)),
        ParserState::Stop(_, _) => {
            panic!("A call to 'parse_params' should never return a 'Stop' result")
        }
        ParserState::Continue(expr, rest) => match *expr {
            Term::Var(_) => {
                let new_result = lambda(rest)?;
                match new_result {
                    ParserState::End(new_rest) => abstr(expr, new_rest),
                    ParserState::Stop(new_expr, new_rest) => {
                        Ok(ParserState::Continue(AST::abstr(expr, new_expr), new_rest))
                    }
                    ParserState::Continue(new_expr, new_rest) => {
                        Ok(ParserState::Continue(AST::abstr(expr, new_expr), new_rest))
                    }
                }
            }
            _ => Err(ParsingError::InvalidLambda(format!(
                "Lambda expression arguments may only contain variables, found {:?}",
                expr
            ))),
        },
    }
}

fn group(tokens: &[Token]) -> Result<ParserState, ParsingError> {
    let result = parse_body(tokens)?;
    match result {
        ParserState::End(_) => Err(ParsingError::MismatchedParens(
            "Empty parentheses '()' are not permitted".to_string(),
        )),
        ParserState::Stop(expr, rest) => Ok(ParserState::Continue(expr, rest)),
        ParserState::Continue(expr, rest) => {
            let new_result = apply(expr, rest)?;
            match new_result {
                ParserState::End(_) => {
                    panic!("Recursive call to 'apply' should never return an 'End' result")
                }
                ParserState::Stop(last_expr, last_rest) => {
                    Ok(ParserState::Continue(last_expr, last_rest))
                }
                ParserState::Continue(_, _) => {
                    panic!("Recursive call to 'apply' should never return a 'Continue' result")
                }
            }
        }
    }
}

pub fn parse(tokens: &[Token]) -> Result<AST, ParsingError> {
    let result = parse_body(tokens)?;
    match result {
        ParserState::End(_) => Err(ParsingError::MismatchedParens(format!(
            "Unexpected ')' at the start of expression"
        ))),
        ParserState::Stop(expr, _) => Ok(expr),
        ParserState::Continue(new_expr, rest) => {
            let new_result = apply(new_expr, rest)?;
            match new_result {
                ParserState::End(_) => {
                    panic!("Recursive call to 'apply' should never return an 'End' result")
                }
                ParserState::Continue(_, _) => {
                    panic!("Recursive call to 'apply' should never return a 'Continue' result")
                }
                ParserState::Stop(last_expr, _) => Ok(last_expr),
            }
        }
    }
}
