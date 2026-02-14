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
#[derive(Debug, PartialEq)]
pub enum ParseError {
    TokenizeError(TokenizeError),
    ParseError(TokenParseError),
}
impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error while parsing JSON Value")
    }
}
impl From<TokenParseError> for ParseError {
    fn from(err: TokenParseError) -> Self {
        Self::ParseError(err)
    }
}
impl From<TokenizeError> for ParseError {
    fn from(err: TokenizeError) -> Self {
        Self::TokenizeError(err)
    }
}

pub struct Config {
    file_path: String,
}
impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("Not enough arguments");
        }
        let file_path = args[1].clone();
        Ok(Config { file_path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    println!("{contents}");
    Ok(())
}
pub fn parse(input: &str) -> Result<Value, ParseError> {
    Ok(Value::String(String::from("diwj")))
}
