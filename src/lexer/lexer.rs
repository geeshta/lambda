use regex::Regex;

#[derive(Debug)]
pub enum Token {
    Var(char),
    Lambda,
    Arrow,
    LParen,
    RParen,
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, regex::Error> {
    let re = Regex::new(r"\$|->|[a-z]|[()]")?;
    Ok(re
        .find_iter(input)
        .filter_map(|mat| match mat.as_str() {
            "$" => Some(Token::Lambda),
            "->" => Some(Token::Arrow),
            "(" => Some(Token::LParen),
            ")" => Some(Token::RParen),
            var if var.chars().all(char::is_lowercase) => var.chars().next().map(Token::Var),
            _ => None,
        })
        .collect())
}
