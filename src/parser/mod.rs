//! Parser module that build an AST from a flat sequence of tokens
pub mod parser;

pub use self::parser::{parse, ParsingError};
