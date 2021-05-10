use easyjson_rs::parser::*;

#[test]
fn null_test(){
    let mut lexer = Lexer::new();
    assert_eq!(lexer.get_json_tokens("null"),Ok(vec![JSONTokens::NULL]));
}
#[test]
fn n_but_less_than_null_test(){
    let mut lexer = Lexer::new();
    assert_eq!(lexer.get_json_tokens("nf"),Err(LexerError::UnexpectedEOFError));
}
#[test]
fn n_but_not_null_test(){
    let mut lexer = Lexer::new();
    assert_eq!(lexer.get_json_tokens("nffddds"),Err(LexerError::UnexpectedTokenError));
}
#[test]
fn true_test(){
    let mut lexer = Lexer::new();
    assert_eq!(lexer.get_json_tokens("true"),Ok(vec![JSONTokens::TRUE]));
}
#[test]
fn t_but_less_than_null_test(){
    let mut lexer = Lexer::new();
    assert_eq!(lexer.get_json_tokens("tf"),Err(LexerError::UnexpectedEOFError));
}
#[test]
fn t_but_not_null_test(){
    let mut lexer = Lexer::new();
    assert_eq!(lexer.get_json_tokens("tffddds"),Err(LexerError::UnexpectedTokenError));
}
#[test]
fn false_test(){
    let mut lexer = Lexer::new();
    assert_eq!(lexer.get_json_tokens("false"),Ok(vec![JSONTokens::FALSE]));
}
#[test]
fn f_but_less_than_null_test(){
    let mut lexer = Lexer::new();
    assert_eq!(lexer.get_json_tokens("fk"),Err(LexerError::UnexpectedEOFError));
}
#[test]
fn f_but_not_null_test(){
    let mut lexer = Lexer::new();
    assert_eq!(lexer.get_json_tokens("falsek"),Err(LexerError::UnexpectedTokenError));
}