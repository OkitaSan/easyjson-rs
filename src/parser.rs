#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum JSONTokens{
    NULL,
    TRUE,
    FALSE,
}
pub struct Lexer{
    cursor:usize,
}
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum LexerError{
    UnexpectedEOFError,
    UnexpectedTokenError,
}
impl std::fmt::Display for LexerError{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            &LexerError::UnexpectedEOFError => write!(f, "UnexpectEOFError"),
            &LexerError::UnexpectedTokenError => write!(f,"UnexpectedTokenError")
        }
    }
}
impl std::error::Error for LexerError{}
impl Lexer {
    pub fn new() -> Lexer{
        Lexer {
            cursor:0
        }
    }
    pub fn get_json_tokens(&mut self,input:&str) -> Result<Vec<JSONTokens>,LexerError>{
        let mut tokens = Vec::new();
        let mut current_character = input.chars();
        while let Some(character) = current_character.next(){
            match character{
                // null
                'n' => {
                    let u = current_character.next().ok_or(LexerError::UnexpectedEOFError)?;
                    let l = current_character.next().ok_or(LexerError::UnexpectedEOFError)?;
                    let l = current_character.next().ok_or(LexerError::UnexpectedEOFError)?;
                    if u == 'u' && l == 'l' && l == 'l' {
                        tokens.push(JSONTokens::NULL);
                    }else{
                        return Err(LexerError::UnexpectedTokenError);
                    }
                }
                // true 
                't' => {
                    let r = current_character.next().ok_or(LexerError::UnexpectedEOFError)?;
                    let u = current_character.next().ok_or(LexerError::UnexpectedEOFError)?;
                    let e = current_character.next().ok_or(LexerError::UnexpectedEOFError)?;
                    if r == 'r' && u == 'u' && e == 'e'{
                        tokens.push(JSONTokens::TRUE);
                    }else{
                        return Err(LexerError::UnexpectedTokenError);
                    }
                }
                // false
                'f' => {
                    let a = current_character.next().ok_or(LexerError::UnexpectedEOFError)?;
                    let l = current_character.next().ok_or(LexerError::UnexpectedEOFError)?;
                    let s = current_character.next().ok_or(LexerError::UnexpectedEOFError)?;
                    let e = current_character.next().ok_or(LexerError::UnexpectedEOFError)?;
                    if a == 'a' && l == 'l' && s == 's' && e == 'e'{
                        tokens.push(JSONTokens::FALSE);
                    }
                }
                // object
                '{' => {

                }
                // number 
                '-'|'0'..='9' => {

                }
                // array 
                '[' => {

                }
                _ => {
                    return Err(LexerError::UnexpectedTokenError)
                }
            }
        }
        Ok(tokens)
    }
}