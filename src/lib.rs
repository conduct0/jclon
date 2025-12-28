use std::error::Error;
use std::fmt;
use std::fs;

mod tokenize;
pub struct Config {
    file_path: String,
}
#[derive(Debug, Clone)]
struct ParseError;
impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error while parsing JSON Value")
    }
}
enum Value {
    Null,
    True,
    False,
    String(String),
    Number(f64),
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

// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn parse_string() {
//         let string_value = "\"test\"";
//         assert_eq!(Ok(Value::String("test".to_string())), parse(&string_value));
//     }
// }
