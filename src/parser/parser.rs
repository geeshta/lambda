use crate::ast::Term;
use crate::ast::AST;
use crate::lexer::Token;

/// Type for errors during parsing
#[derive(Debug)]
pub enum ParsingError {
    MismatchedParens(String),
    InvalidSyntax(String),
    InvalidLambda(String),
}

/// Type represents different states of the parser
///  - *Continue*: represents an intermediate result of parsing
///  - *Stop*: represents the final result of paring
///  - *End*: represents that end of an expression has been reached
#[derive(Debug)]
pub enum ParserState<'a> {
    Continue(AST, &'a [Token]),
    Stop(AST, &'a [Token]),
    End(&'a [Token]),
}

/// Base parser that parses valid lambda expressions
fn parse_expression(tokens: &[Token]) -> Result<ParserState, ParsingError> {
    match tokens {
        // Create a variable node and continue parsing
        [Token::Var(s), rest @ ..] => Ok(ParserState::Continue(AST::var(s.clone()), rest)),
        // Opening parenthesis -> open a group
        [Token::LParen, rest @ ..] => group(rest),
        // Start of a lambda expression -> start parsing parameters
        [Token::Lambda, rest @ ..] => lambda(rest),
        // Right parenthesis -> signal end of expression
        [Token::RParen, rest @ ..] => Ok(ParserState::End(rest)),
        [] => Ok(ParserState::End(&[])),
        _ => Err(ParsingError::InvalidSyntax(format!(
            "Unexpected token in expression body: {:?}",
            tokens[0]
        ))),
    }
}

/// Parser handling the parameters of a lambda expression
fn parse_params(tokens: &[Token]) -> Result<ParserState, ParsingError> {
    match tokens {
        // Variable and a comma -> continue processing parameters
        [Token::Var(s), Token::Comma, rest @ ..] => {
            Ok(ParserState::Continue(AST::var(s.clone()), rest))
        }
        // Variable and an arrow -> stop processing parameters
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

/// Function handling building of application terms
/// It uses recursion to handle chained applications
fn apply(term: AST, tokens: &[Token]) -> Result<ParserState, ParsingError> {
    let result = parse_expression(tokens)?;
    match result {
        // Right parens found -> return the left term (typically a variable)
        ParserState::End(rest) => Ok(ParserState::Stop(term, rest)),
        // Right term stopped evaluating -> build the application from the two terms
        ParserState::Stop(new_term, rest) => {
            Ok(ParserState::Stop(AST::apply(term, new_term), rest))
        }
        // Intermediate result -> recurse to process more tokens
        ParserState::Continue(new_term, rest) => {
            let new_result = apply(AST::apply(term, new_term), rest)?;
            match new_result {
                // Apply has no path where it return a "Stop"
                ParserState::End(_) => {
                    panic!("Recursive call to 'apply' should never return an 'End' result")
                }
                // Stopped processing on the recursive call -> pass the resulting term to the caller
                ParserState::Stop(final_term, final_rest) => {
                    Ok(ParserState::Stop(final_term, final_rest))
                }
                // Still no final result -> continue recursing
                ParserState::Continue(final_term, last_rest) => apply(final_term, last_rest),
            }
        }
    }
}

/// Function handling chaining parameters recursively, i.e.
/// $xy -> x => $x -> $y -> x
fn lambda(tokens: &[Token]) -> Result<ParserState, ParsingError> {
    let result = parse_params(tokens)?;
    match result {
        // Parse_params has no path where it return a "End"
        ParserState::End(_) => {
            panic!("A call to 'parse_params' should never return a 'End' result")
        }
        // Found the final parameter -> start building the abstraction body
        ParserState::Stop(term, rest) => abstr(term, rest),
        // A parameter ends with a coma -> recursively process more
        ParserState::Continue(term, rest) => match term.term {
            Term::Var(_) => {
                let new_result = lambda(rest)?;
                match new_result {
                    // Lambda has no path where it return a "End"
                    ParserState::End(_) => {
                        panic!("A call to 'lambda' should never return a 'End' result")
                    }
                    // Found the final parameter -> prepare the "container" abstraction
                    ParserState::Stop(new_term, new_rest) => {
                        Ok(ParserState::Continue(AST::abstr(term, new_term), new_rest))
                    }
                    // Found the next parameter -> prepare the "container" abstraction
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

/// Function handling building of abstraction terms
/// It recurses into apply() to read the whole lambda body
fn abstr(arg: AST, tokens: &[Token]) -> Result<ParserState, ParsingError> {
    let result = parse_expression(tokens)?;
    match result {
        // Lamdba body can't start with a right paren
        ParserState::End(_) => Err(ParsingError::MismatchedParens(format!(
            "Unexpected ')' at the start of a lambda expression after {:?}",
            arg
        ))),
        // Body finished parsing -> return the lambda with the term as the body
        ParserState::Stop(new_term, rest) => Ok(ParserState::Stop(AST::abstr(arg, new_term), rest)),
        // If the body did not finish, it means it is an application of multiple terms
        ParserState::Continue(new_term, rest) => {
            let new_result = apply(new_term, rest)?;
            match new_result {
                // Apply has no path where it return an "End"
                ParserState::End(_) => {
                    panic!("Recursive call to 'apply' should never return an 'End' result")
                }
                // Processing of body finished -> return the final abstraction
                ParserState::Stop(final_term, final_rest) => {
                    Ok(ParserState::Stop(AST::abstr(arg, final_term), final_rest))
                }
                // Apply has no path where it return a "Continue"
                ParserState::Continue(_, _) => {
                    panic!("Recursive call to 'apply' should never return a 'Continue' result",)
                }
            }
        }
    }
}

/// Function that handles expressions grouped in parentheses ()
/// It recurses into apply() to read the entire expression
fn group(tokens: &[Token]) -> Result<ParserState, ParsingError> {
    let result = parse_expression(tokens)?;
    match result {
        ParserState::End(_) => Err(ParsingError::MismatchedParens(format!(
            "Empty parentheses '()' are not permitted"
        ))),
        // If the final expression is found, turn it into "Continue" so the caller can read more tokens
        ParserState::Stop(term, rest) => Ok(ParserState::Continue(term, rest)),
        // If the final expression is not found, use "apply" to build it from more tokens
        ParserState::Continue(term, rest) => {
            let new_result = apply(term, rest)?;
            match new_result {
                // Apply has no path where it return an "End"
                ParserState::End(_) => {
                    panic!("Recursive call to 'apply' should never return an 'End' result")
                }
                // If the final expression is found, turn it into "Continue" so the caller can read more tokens
                ParserState::Stop(final_term, final_rest) => {
                    Ok(ParserState::Continue(final_term, final_rest))
                }
                // Apply has no path where it return a "Continue"
                ParserState::Continue(_, _) => {
                    panic!("Recursive call to 'apply' should never return a 'Continue' result")
                }
            }
        }
    }
}

/// Main parsing function. It starts with the first expression and then recurses into
/// apply() to read the entire expression
pub fn parse(tokens: &[Token]) -> Result<AST, ParsingError> {
    let result = parse_expression(tokens)?;
    match result {
        ParserState::End(_) => Err(ParsingError::MismatchedParens(format!(
            "Unexpected ')' at the start of expression"
        ))),
        // If the result is found, return it
        ParserState::Stop(term, _) => Ok(term),
        // Otherwise use apply to chain more expressions
        ParserState::Continue(new_term, rest) => {
            let new_result = apply(new_term, rest)?;
            match new_result {
                // Apply has no path where it return an "End"
                ParserState::End(_) => {
                    panic!("Recursive call to 'apply' should never return an 'End' result")
                }
                // Finished AST has been found
                ParserState::Stop(final_term, _) => Ok(final_term),
                // Apply has no path where it return a "Continue"
                ParserState::Continue(_, _) => {
                    panic!("Recursive call to 'apply' should never return a 'Continue' result")
                }
            }
        }
    }
}
