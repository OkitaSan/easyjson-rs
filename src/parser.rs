use core::num;

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
            &LexerError::InvalidNumberFormatError => write!(f,"InvalidNumberFormatError"),
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
        let mut current_character = input.chars();
        let mut brace_matcher = 0;
        let mut bracket_matcher = 0;
        while let Some(character) = current_character.next() {
            match character {
                ' ' | '\t' | '\n' | '\r' => {
                    continue;
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
                    let mut prev_char = 'j';
                    while let Some(digit) = current_character.next() {
                        prev_char = digit;
                        if !digit.is_alphanumeric() {
                            break;
                        } else {
                            number.push(digit);
                        }
                    }
                    if (number[0] == '-' && number.len() == 1)
                        || (number[0] == '0' && number.len() != 1)
                        || (number[0] == '-' && number[1] == '0' && number.len() != 2)
                    {
                        return Err(LexerError::InvalidNumberFormatError);
                    }

                    if prev_char == '.' {
                        number.push(prev_char);
                        let mut cnt = 0;
                        while let Some(digit) = current_character.next() {
                            prev_char = digit;
                            cnt += 1;
                            if !digit.is_alphanumeric() {
                                break;
                            } else {
                                number.push(digit);
                            }
                        }
                        if cnt == 0 {
                            return Err(LexerError::ExpectedDigitAfterPointError);
                        }
                    }
                    if prev_char == 'e' || prev_char == 'E' {
                        number.push(prev_char);
                        // TODO: 监视剩下的数字，可以利用bool flag和prev char来侦测0
                        while let Some(val) = current_character.next(){
                            if val == '+' || val == '-'{
                                number.push(val);
                            }
                        }
                    }
                }
                _ => return Err(LexerError::UnexpectedTokenError),
            }
        }
        Ok(tokens)
    }
}
