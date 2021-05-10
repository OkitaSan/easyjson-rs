use easyjson_rs::parser::*;

#[test]
fn null_test(){
    let mut lexer = Lexer::new();
    assert_eq!(lexer.get_json_tokens("null"),Ok(vec![JSONTokens::Null]));
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
    assert_eq!(lexer.get_json_tokens("true"),Ok(vec![JSONTokens::True]));
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
    assert_eq!(lexer.get_json_tokens("false"),Ok(vec![JSONTokens::False]));
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
#[test]
fn number_ok_test(){
    let mut lexer = Lexer::new();
    assert_eq!(lexer.get_json_tokens("1234"),Ok(vec![JSONTokens::Number(1234f64)]));
    assert_eq!(lexer.get_json_tokens("-1234"),Ok(vec![JSONTokens::Number(-1234f64)]));
    assert_eq!(lexer.get_json_tokens("12.34"),Ok(vec![JSONTokens::Number(12.34)]));
    assert_eq!(lexer.get_json_tokens("12.34e2"),Ok(vec![JSONTokens::Number(12.34e2)]));
    assert_eq!(lexer.get_json_tokens("12.34E3"),Ok(vec![JSONTokens::Number(12.34E3)]));
    assert_eq!(lexer.get_json_tokens("12e2"),Ok(vec![JSONTokens::Number(12e2)]));
    assert_eq!(lexer.get_json_tokens("12E2"),Ok(vec![JSONTokens::Number(12E2)]));
    assert_eq!(lexer.get_json_tokens("-0"),Ok(vec![JSONTokens::Number(0f64)]));
    assert_eq!(lexer.get_json_tokens("0"),Ok(vec![JSONTokens::Number(0f64)]));
    assert_eq!(lexer.get_json_tokens("-1234"),Ok(vec![JSONTokens::Number(-1234f64)]));
    assert_eq!(lexer.get_json_tokens("-12.34"),Ok(vec![JSONTokens::Number(-12.34)]));
    assert_eq!(lexer.get_json_tokens("-12.34e2"),Ok(vec![JSONTokens::Number(-12.34e2)]));
    assert_eq!(lexer.get_json_tokens("-12.34E3"),Ok(vec![JSONTokens::Number(-12.34E3)]));
    assert_eq!(lexer.get_json_tokens("-12e2"),Ok(vec![JSONTokens::Number(-12e2)]));
    assert_eq!(lexer.get_json_tokens("-12E2"),Ok(vec![JSONTokens::Number(-12E2)]));
    assert_eq!(lexer.get_json_tokens("12.34e+2"),Ok(vec![JSONTokens::Number(12.34e2)]));
    assert_eq!(lexer.get_json_tokens("12.34E+3"),Ok(vec![JSONTokens::Number(12.34E3)]));
    assert_eq!(lexer.get_json_tokens("12e+2"),Ok(vec![JSONTokens::Number(12e2)]));
    assert_eq!(lexer.get_json_tokens("12E+2"),Ok(vec![JSONTokens::Number(12E2)]));
    assert_eq!(lexer.get_json_tokens("-12.34e+2"),Ok(vec![JSONTokens::Number(-12.34e2)]));
    assert_eq!(lexer.get_json_tokens("-12.34E+3"),Ok(vec![JSONTokens::Number(-12.34E3)]));
    assert_eq!(lexer.get_json_tokens("-12e+2"),Ok(vec![JSONTokens::Number(-12e2)]));
    assert_eq!(lexer.get_json_tokens("-12E+2"),Ok(vec![JSONTokens::Number(-12E2)]));
    assert_eq!(lexer.get_json_tokens("12.34e-2"),Ok(vec![JSONTokens::Number(12.34e-2)]));
    assert_eq!(lexer.get_json_tokens("12.34E-3"),Ok(vec![JSONTokens::Number(12.34E-3)]));
    assert_eq!(lexer.get_json_tokens("12e-2"),Ok(vec![JSONTokens::Number(12e-2)]));
    assert_eq!(lexer.get_json_tokens("12E-2"),Ok(vec![JSONTokens::Number(12E-2)]));
    assert_eq!(lexer.get_json_tokens("-12.34e-2"),Ok(vec![JSONTokens::Number(-12.34e-2)]));
    assert_eq!(lexer.get_json_tokens("-12.34E-3"),Ok(vec![JSONTokens::Number(-12.34E-3)]));
    assert_eq!(lexer.get_json_tokens("-12e-2"),Ok(vec![JSONTokens::Number(-12e-2)]));
    assert_eq!(lexer.get_json_tokens("-12E-2"),Ok(vec![JSONTokens::Number(-12E-2)]));
    assert_eq!(lexer.get_json_tokens("-12E-2"),Ok(vec![JSONTokens::Number(-12E-2)]));
    assert_eq!(lexer.get_json_tokens("-12,23"),Ok(vec![JSONTokens::Number(-12f64),JSONTokens::Comma,JSONTokens::Number(23f64)]));
}