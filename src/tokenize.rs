use std::fmt;

use std::num::ParseFloatError;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TokenizeError {
    UnfinishedLiteralValue,
    ParseNumberError(ParseFloatError),
    UnclosedQuotes,
    UnexpectedEof,
    CharNotRecognized(char),
}
#[derive(Debug, PartialEq)]
pub enum Token {
    CLeftBracket,
    CRightBracket,
    SLeftBracket,
    SRightBracket,
    Comma,
    Colon,
    Null,
    True,
    False,
    String(String),
    Number(f64),
}

#[cfg(test)]
impl Token {
    pub fn string(input: &str) -> Self {
        Self::String(String::from(input))
    }
}
impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Null => write!(f, "null"),
            Token::True => write!(f, "true"),
            Token::False => write!(f, "false"),
            _ => todo!("No string representation yet for {self}"),
        }
    }
}
pub fn tokenize(input: String) -> Result<Vec<Token>, TokenizeError> {
    let chars: Vec<char> = input.chars().collect();
    let mut index = 0;
    let mut tokens = Vec::new();

    while index < chars.len() {
        let token = make_token(&chars, &mut index)?;
        tokens.push(token);

        index += 1;
    }
    Ok(tokens)
}

fn make_token(chars: &Vec<char>, index: &mut usize) -> Result<Token, TokenizeError> {
    let mut ch = chars[*index];

    while ch.is_ascii_whitespace() {
        *index += 1;
        if *index >= chars.len() {
            return Err(TokenizeError::UnexpectedEof);
        }
        ch = chars[*index];
    }
    let token = match ch {
        '{' => Token::CLeftBracket,
        '}' => Token::CRightBracket,
        '[' => Token::SLeftBracket,
        ']' => Token::SRightBracket,
        ',' => Token::Comma,
        ':' => Token::Colon,
        'n' => tokenize_literal(chars, index, Token::Null)?,
        't' => tokenize_literal(chars, index, Token::True)?,
        'f' => tokenize_literal(chars, index, Token::False)?,
        '"' => tokenize_string(chars, index)?,
        ch if ch.is_ascii_digit() || ch == '-' => tokenize_float(chars, index)?,
        ch => return Err(TokenizeError::CharNotRecognized(ch)),
    };
    Ok(token)
}
fn tokenize_string(chars: &Vec<char>, index: &mut usize) -> Result<Token, TokenizeError> {
    let mut str = String::new();
    let mut is_escaping = false;

    loop {
        *index += 1;
        if *index >= chars.len() {
            return Err(TokenizeError::UnclosedQuotes);
        }
        let ch = chars[*index];
        match ch {
            '"' if !is_escaping => break,
            '\\' => is_escaping = !is_escaping,
            _ => is_escaping = false,
        }
        str.push(ch);
    }
    Ok(Token::String(str))
}

fn tokenize_literal(
    chars: &Vec<char>,
    index: &mut usize,
    token: Token,
) -> Result<Token, TokenizeError> {
    for expected_char in token.to_string().chars() {
        if expected_char != chars[*index] {
            return Err(TokenizeError::UnfinishedLiteralValue);
        }
        *index += 1;
    }
    *index -= 1;
    Ok(token)
}
fn tokenize_float(chars: &Vec<char>, index: &mut usize) -> Result<Token, TokenizeError> {
    let mut unparsed_num = String::new();
    let mut has_decimal = false;

    if chars[*index] == '-' {
        unparsed_num.push(chars[*index]);
        *index += 1;
    }
    while *index < chars.len() {
        let ch = chars[*index];

        match ch {
            ch if ch.is_ascii_digit() => unparsed_num.push(ch),
            ch if ch == '.' && !has_decimal => {
                has_decimal = true;
                unparsed_num.push(ch)
            }
            _ => break,
        }
        *index += 1;
    }
    *index -= 1;
    match unparsed_num.parse() {
        Ok(f) => Ok(Token::Number(f)),
        Err(err) => Err(TokenizeError::ParseNumberError(err)),
    }
}

#[cfg(test)]
mod tests {

    use crate::tokenize::TokenizeError;

    use super::{Token, tokenize};

    #[test]
    fn just_comma() {
        let input = String::from(",");
        let expected = [Token::Comma];

        let actual = tokenize(input).unwrap();
        assert_eq!(actual, expected);
    }
    #[test]
    fn true_comma() {
        let input = String::from("true,");
        let expected = [Token::True, Token::Comma];

        let actual = tokenize(input).unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn just_null() {
        let input = String::from("null");
        let expected = [Token::Null];

        let actual = tokenize(input).unwrap();
        assert_eq!(actual, expected);
    }
    #[test]
    fn just_true() {
        let input = String::from("true");
        let expected = [Token::True];

        let actual = tokenize(input).unwrap();
        assert_eq!(actual, expected);
    }
    #[test]
    fn just_false() {
        let input = String::from("false");
        let expected = [Token::False];

        let actual = tokenize(input).unwrap();
        assert_eq!(actual, expected);
    }
    #[test]
    fn integer() {
        let input = String::from("123");
        let expected = [Token::Number(123.0)];

        let actual = tokenize(input).unwrap();
        assert_eq!(actual, expected);
    }
    #[test]
    fn floating_point() {
        let input = String::from("123.123");
        let expected = [Token::Number(123.123)];

        let actual = tokenize(input).unwrap();
        assert_eq!(actual, expected);
    }
    #[test]
    fn negative_number() {
        let input = String::from("-12");

        let expected = [Token::Number(-12.0)];

        let actual = tokenize(input).unwrap();
        assert_eq!(actual, expected);
    }
    #[test]
    fn just_ken() {
        let input = String::from("\"ken\"");

        let expected = [Token::string("ken")];

        let actual = tokenize(input).unwrap();
        assert_eq!(actual, expected);
    }
    #[test]
    fn just_ken_bad() {
        let input = String::from("\"ken");
        let expected = Err(TokenizeError::UnclosedQuotes);

        let actual = tokenize(input);
        assert_eq!(actual, expected)
    }
    #[test]
    fn escaped_quote() {
        let input = String::from(r#""this is \" escaped""#);
        let expected = [Token::string(r#"this is \" escaped"#)];

        let actual = tokenize(input).unwrap();
        assert_eq!(actual, expected)
    }
    #[test]
    fn unkown_char() {
        let input = String::from(r#"&"#);
        let expected = Err(TokenizeError::CharNotRecognized('&'));

        let actual = tokenize(input);
        assert_eq!(actual, expected)
    }
}
