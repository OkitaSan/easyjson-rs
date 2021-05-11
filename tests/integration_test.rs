use std::ops::Add;

use easyjson_rs::lexer::*;
use easyjson_rs::useful_kt_extensions::*;
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

#[test]
fn number_invalid_zero(){
    let mut lexer = Lexer::new();
    assert_eq!(lexer.get_json_tokens("-012"),Err(LexerError::InvalidNumberFormatError));
    assert_eq!(lexer.get_json_tokens("012"),Err(LexerError::InvalidNumberFormatError));
}

#[test]
fn negative_sign_without_number(){
    let mut lexer = Lexer::new();
    assert_eq!(lexer.get_json_tokens("-"),Err(LexerError::InvalidNumberFormatError));
}

#[test]
fn number_has_point_but_no_digits_after_point(){
    let mut lexer = Lexer::new();
    assert_eq!(lexer.get_json_tokens("-32."),Err(LexerError::ExpectedDigitAfterPointError));
    assert_eq!(lexer.get_json_tokens("32."),Err(LexerError::ExpectedDigitAfterPointError));
    assert_eq!(lexer.get_json_tokens("-0."),Err(LexerError::ExpectedDigitAfterPointError));
    assert_eq!(lexer.get_json_tokens("0."),Err(LexerError::ExpectedDigitAfterPointError));
}

#[test]
fn exponent_without_digit(){
    let mut lexer = Lexer::new();
    assert_eq!(lexer.get_json_tokens("12e"),Err(LexerError::InvalidNumberFormatError));
    assert_eq!(lexer.get_json_tokens("-12e"),Err(LexerError::InvalidNumberFormatError));
    assert_eq!(lexer.get_json_tokens("12.1e"),Err(LexerError::InvalidNumberFormatError));
    assert_eq!(lexer.get_json_tokens("-12.1e"),Err(LexerError::InvalidNumberFormatError));
    assert_eq!(lexer.get_json_tokens("0e"),Err(LexerError::InvalidNumberFormatError));
    assert_eq!(lexer.get_json_tokens("-0e"),Err(LexerError::InvalidNumberFormatError));
    assert_eq!(lexer.get_json_tokens("12E"),Err(LexerError::InvalidNumberFormatError));
    assert_eq!(lexer.get_json_tokens("-12E"),Err(LexerError::InvalidNumberFormatError));
    assert_eq!(lexer.get_json_tokens("12.1E"),Err(LexerError::InvalidNumberFormatError));
    assert_eq!(lexer.get_json_tokens("-12.1E"),Err(LexerError::InvalidNumberFormatError));
    assert_eq!(lexer.get_json_tokens("0E"),Err(LexerError::InvalidNumberFormatError));
    assert_eq!(lexer.get_json_tokens("-0E"),Err(LexerError::InvalidNumberFormatError));

    assert_eq!(lexer.get_json_tokens("12e+"),Err(LexerError::InvalidNumberFormatError));
    assert_eq!(lexer.get_json_tokens("-12e+"),Err(LexerError::InvalidNumberFormatError));
    assert_eq!(lexer.get_json_tokens("12.1e+"),Err(LexerError::InvalidNumberFormatError));
    assert_eq!(lexer.get_json_tokens("-12.1e+"),Err(LexerError::InvalidNumberFormatError));
    assert_eq!(lexer.get_json_tokens("0e+"),Err(LexerError::InvalidNumberFormatError));
    assert_eq!(lexer.get_json_tokens("-0e+"),Err(LexerError::InvalidNumberFormatError));
    assert_eq!(lexer.get_json_tokens("12E+"),Err(LexerError::InvalidNumberFormatError));
    assert_eq!(lexer.get_json_tokens("-12E+"),Err(LexerError::InvalidNumberFormatError));
    assert_eq!(lexer.get_json_tokens("12.1E+"),Err(LexerError::InvalidNumberFormatError));
    assert_eq!(lexer.get_json_tokens("-12.1E+"),Err(LexerError::InvalidNumberFormatError));
    assert_eq!(lexer.get_json_tokens("0E+"),Err(LexerError::InvalidNumberFormatError));
    assert_eq!(lexer.get_json_tokens("-0E+"),Err(LexerError::InvalidNumberFormatError));

    assert_eq!(lexer.get_json_tokens("12e-"),Err(LexerError::InvalidNumberFormatError));
    assert_eq!(lexer.get_json_tokens("-12e-"),Err(LexerError::InvalidNumberFormatError));
    assert_eq!(lexer.get_json_tokens("12.1e-"),Err(LexerError::InvalidNumberFormatError));
    assert_eq!(lexer.get_json_tokens("-12.1e-"),Err(LexerError::InvalidNumberFormatError));
    assert_eq!(lexer.get_json_tokens("0e-"),Err(LexerError::InvalidNumberFormatError));
    assert_eq!(lexer.get_json_tokens("-0e-"),Err(LexerError::InvalidNumberFormatError));
    assert_eq!(lexer.get_json_tokens("12E-"),Err(LexerError::InvalidNumberFormatError));
    assert_eq!(lexer.get_json_tokens("-12E-"),Err(LexerError::InvalidNumberFormatError));
    assert_eq!(lexer.get_json_tokens("12.1E-"),Err(LexerError::InvalidNumberFormatError));
    assert_eq!(lexer.get_json_tokens("-12.1E-"),Err(LexerError::InvalidNumberFormatError));
    assert_eq!(lexer.get_json_tokens("0E-"),Err(LexerError::InvalidNumberFormatError));
    assert_eq!(lexer.get_json_tokens("-0E-"),Err(LexerError::InvalidNumberFormatError));
}

#[test]
fn exponent_has_digit_after_zero(){
    let mut lexer = Lexer::new();
    assert_eq!(lexer.get_json_tokens("12e01"),Err(LexerError::InvalidNumberFormatError));
    assert_eq!(lexer.get_json_tokens("-12e01"),Err(LexerError::InvalidNumberFormatError));
    assert_eq!(lexer.get_json_tokens("12.1e01"),Err(LexerError::InvalidNumberFormatError));
    assert_eq!(lexer.get_json_tokens("-12.1e01"),Err(LexerError::InvalidNumberFormatError));
    assert_eq!(lexer.get_json_tokens("0e01"),Err(LexerError::InvalidNumberFormatError));
    assert_eq!(lexer.get_json_tokens("-0e01"),Err(LexerError::InvalidNumberFormatError));
    assert_eq!(lexer.get_json_tokens("12E01"),Err(LexerError::InvalidNumberFormatError));
    assert_eq!(lexer.get_json_tokens("-12E01"),Err(LexerError::InvalidNumberFormatError));
    assert_eq!(lexer.get_json_tokens("12.1E01"),Err(LexerError::InvalidNumberFormatError));
    assert_eq!(lexer.get_json_tokens("-12.1E01"),Err(LexerError::InvalidNumberFormatError));
    assert_eq!(lexer.get_json_tokens("0E01"),Err(LexerError::InvalidNumberFormatError));
    assert_eq!(lexer.get_json_tokens("-0E01"),Err(LexerError::InvalidNumberFormatError));

    assert_eq!(lexer.get_json_tokens("12e+01"),Err(LexerError::InvalidNumberFormatError));
    assert_eq!(lexer.get_json_tokens("-12e+01"),Err(LexerError::InvalidNumberFormatError));
    assert_eq!(lexer.get_json_tokens("12.1e+01"),Err(LexerError::InvalidNumberFormatError));
    assert_eq!(lexer.get_json_tokens("-12.1e+01"),Err(LexerError::InvalidNumberFormatError));
    assert_eq!(lexer.get_json_tokens("0e+01"),Err(LexerError::InvalidNumberFormatError));
    assert_eq!(lexer.get_json_tokens("-0e+01"),Err(LexerError::InvalidNumberFormatError));
    assert_eq!(lexer.get_json_tokens("12E+01"),Err(LexerError::InvalidNumberFormatError));
    assert_eq!(lexer.get_json_tokens("-12E+01"),Err(LexerError::InvalidNumberFormatError));
    assert_eq!(lexer.get_json_tokens("12.1E+01"),Err(LexerError::InvalidNumberFormatError));
    assert_eq!(lexer.get_json_tokens("-12.1E+01"),Err(LexerError::InvalidNumberFormatError));
    assert_eq!(lexer.get_json_tokens("0E+01"),Err(LexerError::InvalidNumberFormatError));
    assert_eq!(lexer.get_json_tokens("-0E+01"),Err(LexerError::InvalidNumberFormatError));

    assert_eq!(lexer.get_json_tokens("12e-01"),Err(LexerError::InvalidNumberFormatError));
    assert_eq!(lexer.get_json_tokens("-12e-01"),Err(LexerError::InvalidNumberFormatError));
    assert_eq!(lexer.get_json_tokens("12.1e-01"),Err(LexerError::InvalidNumberFormatError));
    assert_eq!(lexer.get_json_tokens("-12.1e-01"),Err(LexerError::InvalidNumberFormatError));
    assert_eq!(lexer.get_json_tokens("0e-01"),Err(LexerError::InvalidNumberFormatError));
    assert_eq!(lexer.get_json_tokens("-0e-01"),Err(LexerError::InvalidNumberFormatError));
    assert_eq!(lexer.get_json_tokens("12E-01"),Err(LexerError::InvalidNumberFormatError));
    assert_eq!(lexer.get_json_tokens("-12E-01"),Err(LexerError::InvalidNumberFormatError));
    assert_eq!(lexer.get_json_tokens("12.1E-01"),Err(LexerError::InvalidNumberFormatError));
    assert_eq!(lexer.get_json_tokens("-12.1E-01"),Err(LexerError::InvalidNumberFormatError));
    assert_eq!(lexer.get_json_tokens("0E-01"),Err(LexerError::InvalidNumberFormatError));
    assert_eq!(lexer.get_json_tokens("-0E-01"),Err(LexerError::InvalidNumberFormatError));
}
#[test]
fn test_whether_implementation_will_consume_token_after_tokenization(){
    let mut k = Lexer::new();
    assert_eq!(k.get_json_tokens("123null"),Ok(vec![JSONTokens::Number(123.0),JSONTokens::Null]));
    assert_eq!(k.get_json_tokens("123o"),Err(LexerError::UnexpectedTokenError));
}
#[test]
fn kotlin_scope_function_let_test(){ 
   let k = 1i32.kotlin_let_mut_ref(|x| x.add(2));
   assert_eq!(k,3);
   let i = "fdsafdsa".kotlin_let_ref(|x|x.len());
   assert_eq!(i,8);
}
#[test]
fn kotlin_scope_function_also_test(){
    let k = 1i32.kotlin_also_ref(|x|println!("{}", x)).add(3);
    assert_eq!(k,4);
    let j = "fda".to_string().kotlin_also_mut_ref(|x| x.push('c')).len();
    assert_eq!(j,4);
}

#[test]
fn test_correct_string(){
    let mut lexer = Lexer::new();
    assert_eq!(lexer.get_json_tokens(r#""""#),Ok(vec![JSONTokens::String(r#""""#.to_string())]));
    assert_eq!(lexer.get_json_tokens(r#"
        "I love you"
    "#),Ok(vec![JSONTokens::String(r#"
        "I love you"
    "#.trim().to_string())]));
    assert_eq!(lexer.get_json_tokens(r#"
        "\u0000"
    "#),Ok(vec![JSONTokens::String(r#"
        "\u0000"
    "#.trim().to_string())]));
    assert_eq!(lexer.get_json_tokens(r#"
        "\uabcd"
    "#),Ok(vec![JSONTokens::String(r#"
        "\uabcd"
    "#.trim().to_string())]));
    assert_eq!(lexer.get_json_tokens(r#"
        "\uABCD"
    "#),Ok(vec![JSONTokens::String(r#"
        "\uABCD"
    "#.trim().to_string())]));
    assert_eq!(lexer.get_json_tokens(r#"
        "\"\\\/\b\f\n\r\t\uabcd"
    "#),Ok(vec![JSONTokens::String(r#"
        "\"\\\/\b\f\n\r\t\uabcd"
    "#.trim().to_string())]));
}

#[test]
fn test_escape_lossing(){
    let mut lexer = Lexer::new();
    assert_eq!(lexer.get_json_tokens(r#"
        "\"\\\/\b\f\n\r\t\uabcd
    "#),Err(LexerError::StringQuotationNotMatchError));
}

#[test]
fn test_invalid_control_character(){
    let mut lexer = Lexer::new();
    assert_eq!(lexer.get_json_tokens(r#"
        "\1"
    "#),Err(LexerError::InvalidControlCharacterError));
    assert_eq!(lexer.get_json_tokens(r#"
        "\2"
    "#),Err(LexerError::InvalidControlCharacterError));
    assert_eq!(lexer.get_json_tokens(r#"
        "\3"
    "#),Err(LexerError::InvalidControlCharacterError));
}

#[test]
fn test_invalid_unicode_digit(){
    let mut lexer = Lexer::new();
    assert_eq!(lexer.get_json_tokens(r#"
        "\ghri"
    "#),Err(LexerError::InvalidControlCharacterError));
}