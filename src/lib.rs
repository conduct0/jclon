use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::fs;

use crate::parse::TokenParseError;
use crate::parse::parse_tokens;
use crate::tokenize::TokenizeError;
use crate::tokenize::tokenize;

mod parse;
mod tokenize;

pub fn parse(input: String) -> Result<Value, ParseError> {
    let tokens = tokenize(input)?;
    let value = parse_tokens(&tokens, &mut 0)?;
    Ok(value)
}

#[derive(Debug, Clone, PartialEq)]
enum Value {
    Null,
    Boolean(bool),
    String(String),
    Number(f64),
    Array(Vec<Value>),
    Object(HashMap<String, Value>),
}
#[cfg(test)]
impl Value {
    pub fn string(input: &str) -> Self {
        Self::String(String::from(input))
    }
}
impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Null => write!(f, "null"),
            Value::Boolean(b) => write!(f, "{b}"),
            Value::String(s) => write!(f, "\"{s}\""),
            Value::Number(n) => write!(f, "{n}"),
            Value::Array(arr) => {
                write!(f, "[")?;
                for (i, item) in arr.iter().enumerate() {
                    if i > 0 {
                        write!(f, ",");
                    }
                    write!(f, "{item}");
                }
                write!(f, "]")
            }
            Value::Object(obj) => {
                write!(f, "{{")?;
                for (i, (key, value)) in obj.iter().enumerate() {
                    if i > 0 {
                        write!(f, ",");
                    }
                    write!(f, "{key}:{value}");
                }
                write!(f, "}}")
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum CliError {
    InvalidArguments,
    ReadFile,
}
impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::InvalidArguments => write!(f, "InvalidArguments"),
            Self::ReadFile => write!(f, "ReadFile"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ParseError {
    Tokenize(TokenizeError),
    Parse(TokenParseError),
    Cli(CliError),
}
impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::Tokenize(e) => write!(f, "TokenizeError {e}"),
            ParseError::Parse(e) => write!(f, "ParseError {e}"),
            ParseError::Cli(e) => write!(f, "CliError {e}"),
        }
    }
}
impl From<TokenParseError> for ParseError {
    fn from(err: TokenParseError) -> Self {
        Self::Parse(err)
    }
}
impl From<TokenizeError> for ParseError {
    fn from(err: TokenizeError) -> Self {
        Self::Tokenize(err)
    }
}
impl From<CliError> for ParseError {
    fn from(err: CliError) -> Self {
        Self::Cli(err)
    }
}

pub struct Config {
    file_path: String,
}
impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, ParseError> {
        // ignore first that is the program name
        args.next();
        let file_path = match args.next() {
            Some(s) => s,
            None => return Err(ParseError::Cli(CliError::InvalidArguments)),
        };
        Ok(Config { file_path })
    }
}

pub fn run(config: Config) -> Result<(), ParseError> {
    let contents =
        fs::read_to_string(config.file_path).map_err(|e| ParseError::Cli(CliError::ReadFile))?;
    let res = parse(contents.clone())?;
    println!("jowjwjojlj  {res}\n");
    Ok(())
}
