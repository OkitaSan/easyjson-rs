use core::num;
use std::{iter::FromIterator, str::FromStr};
use crate::useful_kt_extensions::*;
#[derive(Debug, Clone, PartialEq)]
pub enum JSONTokens {
    // '{'
    OpenBrace,
    // '}'
    CloseBrace,
    // '['
    OpenBracket,
    // ']'
    CloseBracket,
    // ','
    Comma,
    // ':'
    Colon,
    // Number
    Number(f64),
    // String
    String(String),
    // null
    Null,
    // true
    True,
    // false
    False,
}

pub struct Lexer {
    cursor: usize,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum LexerError {
    UnexpectedEOFError,
    UnexpectedTokenError,
    BraceNotMatchError,
    BracketNotMatchError,
    ExpectedDigitAfterPointError,
    InvalidNumberFormatError,
}

impl std::fmt::Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            &LexerError::UnexpectedEOFError => write!(f, "UnexpectEOFError"),
            &LexerError::UnexpectedTokenError => write!(f, "UnexpectedTokenError"),
            &LexerError::BraceNotMatchError => write!(f, "BraceNotMatchError"),
            &LexerError::BracketNotMatchError => write!(f, "BracketNotMatchError"),
            &LexerError::ExpectedDigitAfterPointError => write!(f, "ExpectedDigitAfterPointError"),
            &LexerError::InvalidNumberFormatError => write!(f, "InvalidNumberFormatError"),
        }
    }
}

impl std::error::Error for LexerError {}

impl Lexer {
    pub fn new() -> Lexer {
        Lexer { cursor: 0 }
    }
    pub fn get_json_tokens(&mut self, input: &str) -> Result<Vec<JSONTokens>, LexerError> {
        let mut tokens = Vec::new();
        let mut current_character = input.chars().peekable();
        let mut brace_matcher = 0;
        let mut bracket_matcher = 0;
        while let Some(character) = current_character.next() {
            match character {
                ' ' | '\t' | '\n' | '\r' => {
                    continue;
                }
                ',' => {
                    tokens.push(JSONTokens::Comma);
                }
                // null
                'n' => {
                    let u = current_character
                        .next()
                        .ok_or(LexerError::UnexpectedEOFError)?;
                    let l = current_character
                        .next()
                        .ok_or(LexerError::UnexpectedEOFError)?;
                    let l = current_character
                        .next()
                        .ok_or(LexerError::UnexpectedEOFError)?;
                    if u == 'u' && l == 'l' && l == 'l' {
                        tokens.push(JSONTokens::Null);
                    } else {
                        return Err(LexerError::UnexpectedTokenError);
                    }
                }
                // true
                't' => {
                    let r = current_character
                        .next()
                        .ok_or(LexerError::UnexpectedEOFError)?;
                    let u = current_character
                        .next()
                        .ok_or(LexerError::UnexpectedEOFError)?;
                    let e = current_character
                        .next()
                        .ok_or(LexerError::UnexpectedEOFError)?;
                    if r == 'r' && u == 'u' && e == 'e' {
                        tokens.push(JSONTokens::True);
                    } else {
                        return Err(LexerError::UnexpectedTokenError);
                    }
                }
                // false
                'f' => {
                    let a = current_character
                        .next()
                        .ok_or(LexerError::UnexpectedEOFError)?;
                    let l = current_character
                        .next()
                        .ok_or(LexerError::UnexpectedEOFError)?;
                    let s = current_character
                        .next()
                        .ok_or(LexerError::UnexpectedEOFError)?;
                    let e = current_character
                        .next()
                        .ok_or(LexerError::UnexpectedEOFError)?;
                    if a == 'a' && l == 'l' && s == 's' && e == 'e' {
                        tokens.push(JSONTokens::False);
                    } else {
                        return Err(LexerError::UnexpectedTokenError);
                    }
                }
                // open brace
                '{' => {
                    brace_matcher += 1;
                    tokens.push(JSONTokens::OpenBrace);
                }
                // close brace
                '}' => {
                    if brace_matcher > 0 {
                        tokens.push(JSONTokens::CloseBrace);
                    } else {
                        return Err(LexerError::BraceNotMatchError);
                    }
                }
                // opem bracket
                '[' => {
                    bracket_matcher += 1;
                    tokens.push(JSONTokens::OpenBracket);
                }
                // close bracket
                ']' => {
                    if bracket_matcher > 0 {
                        tokens.push(JSONTokens::CloseBracket);
                    } else {
                        return Err(LexerError::BracketNotMatchError);
                    }
                }
                // number
                '-' | '0'..='9' => {
                    let mut number = vec![character];
                    while let Some(digit) = current_character.next_if(|x| x.is_numeric()) {
                        number.push(digit);
                    }
                    if (number[0] == '-' && number.len() == 1)
                        || (number[0] == '0' && number.len() != 1)
                        || (number[0] == '-' && number[1] == '0' && number.len() != 2)
                    {
                        return Err(LexerError::InvalidNumberFormatError);
                    }
                    let mut current_number_character = match current_character
                        .next_if(|x| x == &'.' || x == &'e' || x == &'E') {
                        Some(val) => val,
                        None => {
                            let number = f64::from_str(&String::from_iter(number.iter())).unwrap();
                            tokens.push(JSONTokens::Number(number));
                            continue;
                        }
                    }
                        ;

                    if current_number_character == '.' {
                        number.push(current_number_character);
                        let mut cnt = 0;
                        while let Some(digit) = current_character.next_if(|x| x.is_numeric()) {
                            cnt += 1;
                            number.push(digit);
                        }

                        if cnt == 0 {
                            return Err(LexerError::ExpectedDigitAfterPointError);
                        }

                        current_number_character = match current_character
                            .next_if(|x| x == &'e' || x == &'E') {
                            Some(val) => val,
                            None => {
                                let number = f64::from_str(&String::from_iter(number.iter())).unwrap();
                                tokens.push(JSONTokens::Number(number));
                                continue;
                            }
                        }
                        ;

                    }
                    if current_number_character == 'e' || current_number_character == 'E' {
                        number.push(current_number_character);
                        let mut has_leading_zero = false;
                        let mut prev_char = current_number_character;
                        let mut exponent_cnt = 0;
                        while let Some(val) = current_character.next_if(|x| x.is_numeric() || x == &'+' || x == &'-' || x == &'e' || x == &'E') {
                            if val == '+' || val == '-' {
                                number.push(val);
                                prev_char = val;
                                continue;
                            }
                            // detect if it is leading zero
                            if val == '0' && (prev_char == '+' || prev_char == '-' || prev_char == 'e' || prev_char == 'E') {
                                has_leading_zero = true;
                                number.push(val);
                                exponent_cnt += 1;
                                prev_char = val;
                                continue;
                            }
                            if val.is_numeric() {
                                number.push(val);
                                prev_char = val;
                                exponent_cnt += 1;
                            } else {
                                break;
                            }
                        }
                        /*
                                            has_leading_zero   exponent_cnt     status
                                            0                  != 0             Ok
                                            0                  == 0             Err
                                            1                  != 1             Err
                                            1                  == 1             Ok
                         */
                        if (!has_leading_zero && exponent_cnt == 0) || (has_leading_zero && exponent_cnt >= 1) {
                            return Err(LexerError::InvalidNumberFormatError);
                        }
                    }
                    let number = f64::from_str(&String::from_iter(number.iter())).unwrap();
                    tokens.push(JSONTokens::Number(number));
                }
                _ => return Err(LexerError::UnexpectedTokenError),
            }
        }
        Ok(tokens)
    }
}
