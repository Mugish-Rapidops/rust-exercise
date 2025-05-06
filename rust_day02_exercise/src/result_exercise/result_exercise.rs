use std::{fs, num::ParseFloatError};

#[derive(Debug)]
pub struct Uservalidate {
    pub username: String,
    pub age: u8,
}

#[derive(Debug, PartialEq)]
pub enum MathError {
    ParseError(ParseFloatError),
    DivisionByZero,
}

pub fn read_file_contents(path: &str) -> Result<String, String> {
    let content = fs::read_to_string(path);
    match content {
        Ok(data) => Ok(data),
        Err(_) => Err("File not found".to_string()),
    }
}

pub fn parse_even_number(s: &str) -> Result<u32, String> {
    let parse_result = s.parse::<u32>();
    match parse_result {
        Ok(val) => {
            if val % 2 == 0 {
                Ok(val)
            } else {
                Err("Number must be even".to_string())
            }
        }
        Err(_) => Err("Invalid number format".to_string()),
    }
}

pub fn validate_user(username: String, age: u8) -> Result<Uservalidate, Vec<String>> {
    let mut validation_error: Vec<String> = Vec::new();

    if !username
        .chars()
        .all(|char| char.is_alphanumeric() || char == '_')
    {
        validation_error.push("Username must be alphanumeric.".to_string());
    }
    if username.len() < 3 || username.len() > 20 {
        validation_error.push("Username must be between 3 and 20 characters.".to_string());
    }
    if age < 13 || age > 120 {
        validation_error.push("Age must be between 13 and 120.".to_string());
    }

    if validation_error.is_empty() {
        Ok(Uservalidate { username, age })
    } else {
        Err(validation_error)
    }
}

pub fn sum_results(a: Result<i32, String>, b: Result<i32, String>) -> Result<i32, String> {
    match a {
        Ok(a) => match b {
            Ok(b) => Ok(a + b),
            Err(e) => Err(e.to_string()),
        },
        Err(e) => Err(e.to_string()),
    }
}

pub fn divide_strings(a: &str, b: &str) -> Result<f64, MathError> {
    let a = a.parse::<f64>().map_err(MathError::ParseError)?;
    let b = b.parse::<f64>().map_err(MathError::ParseError)?;

    if b == 0.0 {
        Err(MathError::DivisionByZero)
    } else {
        Ok(a / b)
    }
}
