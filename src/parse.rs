use std::collections::HashMap;
use std::fmt;

use crate::Value;
use crate::tokenize::Token;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TokenParseError {
    UnfinishedEscape,
    InvalidHexDigit,
    InvalidCodePointValue,
    ExpectedComma,
    ExpectedColon,
    ExpectedProperty,
}

impl fmt::Display for TokenParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::UnfinishedEscape => write!(f, "UnfinishedEscape"),
            Self::InvalidHexDigit => write!(f, "InvalidHexDigit"),
            Self::InvalidCodePointValue => write!(f, "InvalidCodePointValue"),
            Self::ExpectedComma => write!(f, "ExpectedComma"),
            Self::ExpectedColon => write!(f, "ExpectedColon"),
            Self::ExpectedProperty => write!(f, "ExpectedProperty"),
        }
    }
}
type ParseResult = Result<Value, TokenParseError>;

pub fn parse_tokens(tokens: &[Token], index: &mut usize) -> ParseResult {
    let mut token = &tokens[*index];
    if matches!(
        token,
        Token::Null | Token::True | Token::False | Token::Number(_) | Token::String(_)
    ) {
        *index += 1;
    }
    match token {
        Token::Null => Ok(Value::Null),
        Token::True => Ok(Value::Boolean(true)),
        Token::False => Ok(Value::Boolean(false)),
        Token::Number(number) => Ok(Value::Number(*number)),
        Token::String(string) => parse_string(string),
        Token::SLeftBracket => parse_array(tokens, index),
        Token::CLeftBracket => parse_object(tokens, index),
        _ => todo!(),
    }
}
fn parse_array(tokens: &[Token], index: &mut usize) -> ParseResult {
    debug_assert!(tokens[*index] == Token::SLeftBracket);
    let mut array: Vec<Value> = Vec::new();
    loop {
        // consume left bracket
        *index += 1;
        if tokens[*index] == Token::SRightBracket {
            break;
        }
        let value = parse_tokens(tokens, index)?;
        array.push(value);

        let token = &tokens[*index];
        match token {
            Token::Comma => {}
            Token::SRightBracket => break,
            _ => return Err(TokenParseError::ExpectedComma),
        }
    }
    *index += 1;

    Ok(Value::Array(array))
}

fn parse_object(tokens: &[Token], index: &mut usize) -> ParseResult {
    debug_assert!(tokens[*index] == Token::CLeftBracket);
    let mut object: HashMap<String, Value> = HashMap::new();

    loop {
        *index += 1;
        if tokens[*index] == Token::CRightBracket {
            break;
        }
        if let Token::String(s) = &tokens[*index] {
            *index += 1;
            if Token::Colon == tokens[*index] {
                *index += 1;

                let key = s.clone();
                let value = parse_tokens(tokens, index)?;
                object.insert(key, value);
            } else {
                return Err(TokenParseError::ExpectedColon);
            }
            match &tokens[*index] {
                Token::Comma => {}
                Token::CRightBracket => break,
                _ => return Err(TokenParseError::ExpectedComma),
            }
        } else {
            return Err(TokenParseError::ExpectedProperty);
        }
    }
    *index += 1;
    print! {"obj {:?}", object}
    Ok(Value::Object(object))
}

fn parse_string(s: &str) -> ParseResult {
    let mut chars = s.chars();
    let mut output = String::with_capacity(s.len());
    let mut is_escaping = false;

    while let Some(next_char) = chars.next() {
        if is_escaping {
            match next_char {
                '"' => output.push('"'),
                '\\' => output.push('\\'),
                'b' => output.push('\u{8}'),
                'f' => output.push('\u{12}'),
                'n' => output.push('\n'),
                'r' => output.push('\r'),
                't' => output.push('\t'),
                'u' => {
                    let mut sum = 0;
                    for i in 0..4 {
                        let next_char = chars.next().ok_or(TokenParseError::UnfinishedEscape)?;
                        let digit = next_char
                            .to_digit(16)
                            .ok_or(TokenParseError::InvalidHexDigit)?;
                        sum += (16u32).pow(3 - i) * digit;
                    }
                    let unescaped_char =
                        char::from_u32(sum).ok_or(TokenParseError::InvalidCodePointValue)?;
                    output.push(unescaped_char);
                }
                _ => output.push(next_char),
            }
            is_escaping = false;
        } else if next_char == '\\' {
            is_escaping = true;
        } else {
            output.push(next_char);
        }
    }
    return Ok(Value::String(output));
}

#[cfg(test)]
mod tests {

    use std::collections::HashMap;

    use crate::Value;
    use crate::tokenize::Token;

    use super::parse_tokens;

    fn check(input: &[Token], expected: Value) {
        let actual = parse_tokens(&input, &mut 0).unwrap();
        assert_eq!(actual, expected);
    }
    #[test]
    fn parses_null() {
        let input = [Token::Null];
        let expected = Value::Null;
        check(&input, expected);
    }
    #[test]
    fn parses_true() {
        let input = [Token::True];
        let expected = Value::Boolean(true);

        check(&input, expected);
    }
    #[test]
    fn parses_false() {
        let input = [Token::False];
        let expected = Value::Boolean(false);

        check(&input, expected);
    }
    #[test]
    fn parses_number() {
        let input = [Token::Number(10_f64)];
        let expected = Value::Number(10_f64);

        check(&input, expected);
    }
    #[test]
    fn parses_string() {
        let input = [Token::string("test")];
        let expected = Value::string("test");

        check(&input, expected);
    }
    #[test]
    fn parses_string_with_escapes() {
        let input = [Token::string(r#""test \" ""#)];
        let expected = Value::string(r#""test " ""#);

        check(&input, expected);
    }
    #[test]
    fn parses_string_with_unicodes() {
        let input = [Token::string(r#""test \u002F ""#)];
        let expected = Value::string(r#""test / ""#);

        check(&input, expected);
    }
    #[test]
    fn parses_array_simple() {
        let input = [Token::SLeftBracket, Token::False, Token::SRightBracket];
        let expected = Value::Array(vec![Value::Boolean(false)]);

        check(&input, expected);
    }
    #[test]
    fn parses_array() {
        let input = [
            Token::SLeftBracket,
            Token::False,
            Token::Comma,
            Token::Number(20_f64),
            Token::SRightBracket,
        ];
        let expected = Value::Array(vec![Value::Boolean(false), Value::Number(20_f64)]);

        check(&input, expected);
    }
    #[test]
    fn parses_empty_array() {
        let input = [Token::SLeftBracket, Token::SRightBracket];
        let expected = Value::Array(vec![]);

        check(&input, expected);
    }
    #[test]
    fn parses_array_in_array() {
        let input = [
            Token::SLeftBracket,
            Token::SLeftBracket,
            Token::False,
            Token::Comma,
            Token::Number(20_f64),
            Token::SRightBracket,
            Token::SRightBracket,
        ];
        let expected = Value::Array(vec![Value::Array(vec![
            Value::Boolean(false),
            Value::Number(20_f64),
        ])]);

        check(&input, expected);
    }
    #[test]
    fn parses_empty_obj() {
        let input = [Token::CLeftBracket, Token::CRightBracket];
        let expected = Value::Object(HashMap::new());

        check(&input, expected);
    }
    #[test]
    fn parses_obj() {
        let input = [
            Token::CLeftBracket,
            Token::string("test_key"),
            Token::Colon,
            Token::Null,
            Token::CRightBracket,
        ];
        let expected = Value::Object(HashMap::from([("test_key".into(), Value::Null)]));

        check(&input, expected);
    }
    #[test]
    fn parses_obj_in_obj() {
        let input = [
            Token::CLeftBracket,
            Token::string("test_key"),
            Token::Colon,
            Token::CLeftBracket,
            Token::string("test_key"),
            Token::Colon,
            Token::string("test_value_inside"),
            Token::CRightBracket,
            Token::CRightBracket,
        ];

        let expected_inside = Value::Object(HashMap::from([(
            "test_key".into(),
            Value::string("test_value_inside"),
        )]));
        let expected = Value::Object(HashMap::from([("test_key".into(), expected_inside)]));

        check(&input, expected);
    }
}
