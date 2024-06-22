use regex::Regex;

#[derive(Debug)]
pub enum Token {
    Var(String),
    Lambda,
    Arrow,
    LParen,
    RParen,
    Comma,
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, regex::Error> {
    let re = Regex::new(r"\$|->|[a-zA-Z_][a-zA-Z_0-9]*|[(),]")?;
    Ok(re
        .find_iter(input)
        .filter_map(|mat| match mat.as_str() {
            "$" => Some(Token::Lambda),
            "->" => Some(Token::Arrow),
            "(" => Some(Token::LParen),
            ")" => Some(Token::RParen),
            "," => Some(Token::Comma),
            var if var.chars().all(|c| c.is_alphanumeric() || c == '_') => {
                Some(Token::Var(var.to_string()))
            }
            _ => None,
        })
        .collect())
}
