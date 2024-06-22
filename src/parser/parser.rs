use crate::ast::Term;
use crate::ast::AST;
use crate::lexer::Token;

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
        [Token::Var(s), rest @ ..] => Ok(ParserState::Continue(AST::var(s.clone()), rest)),
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
        [Token::Var(s), Token::Comma, rest @ ..] => {
            Ok(ParserState::Continue(AST::var(s.clone()), rest))
        }
        [Token::Var(s), Token::Arrow, rest @ ..] => {
            Ok(ParserState::Stop(AST::var(s.clone()), rest))
        }
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
        ParserState::Stop(new_term, rest) => {
            Ok(ParserState::Stop(AST::apply(term, new_term), rest))
        }
        ParserState::Continue(new_term, rest) => {
            let new_result = apply(AST::apply(term, new_term), rest)?;
            match new_result {
                ParserState::End(_) => {
                    panic!("Recursive call to 'apply' should never return an 'End' result")
                }
                ParserState::Stop(final_term, final_rest) => {
                    Ok(ParserState::Stop(final_term, final_rest))
                }
                ParserState::Continue(final_term, last_rest) => apply(final_term, last_rest),
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
        ParserState::Stop(new_term, rest) => Ok(ParserState::Stop(AST::abstr(arg, new_term), rest)),
        ParserState::Continue(new_term, rest) => {
            let new_result = apply(new_term, rest)?;
            match new_result {
                ParserState::End(_) => {
                    panic!("Recursive call to 'apply' should never return an 'End' result")
                }
                ParserState::Stop(final_term, final_rest) => {
                    Ok(ParserState::Stop(AST::abstr(arg, final_term), final_rest))
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
        ParserState::End(_) => {
            panic!("A call to 'parse_params' should never return a 'End' result")
        }
        ParserState::Stop(term, rest) => abstr(term, rest),
        ParserState::Continue(term, rest) => match term.term {
            Term::Var(_) => {
                let new_result = lambda(rest)?;
                match new_result {
                    ParserState::End(_) => {
                        panic!("A call to 'parse_params' should never return a 'End' result")
                    }
                    ParserState::Stop(new_term, new_rest) => {
                        Ok(ParserState::Continue(AST::abstr(term, new_term), new_rest))
                    }
                    ParserState::Continue(new_term, new_rest) => {
                        Ok(ParserState::Continue(AST::abstr(term, new_term), new_rest))
                    }
                }
            }
            _ => Err(ParsingError::InvalidLambda(format!(
                "Lambda expression arguments may only contain variables, found {:?}",
                term
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
        ParserState::Stop(term, rest) => Ok(ParserState::Continue(term, rest)),
        ParserState::Continue(term, rest) => {
            let new_result = apply(term, rest)?;
            match new_result {
                ParserState::End(_) => {
                    panic!("Recursive call to 'apply' should never return an 'End' result")
                }
                ParserState::Stop(final_term, final_rest) => {
                    Ok(ParserState::Continue(final_term, final_rest))
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
        ParserState::Stop(term, _) => Ok(term),
        ParserState::Continue(new_term, rest) => {
            let new_result = apply(new_term, rest)?;
            match new_result {
                ParserState::End(_) => {
                    panic!("Recursive call to 'apply' should never return an 'End' result")
                }
                ParserState::Continue(_, _) => {
                    panic!("Recursive call to 'apply' should never return a 'Continue' result")
                }
                ParserState::Stop(final_term, _) => Ok(final_term),
            }
        }
    }
}
