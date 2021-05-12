use std::{future::Ready, iter::Peekable, slice::Iter, vec::IntoIter};

use super::lexer::*;
use super::useful_kt_extensions::*;
#[derive(Debug, Clone)]
pub struct Json {
    value: JsonValue,
}
#[derive(Debug, Clone, PartialEq)]
pub enum JsonValue {
    String(String),
    Number(f64),
    Object(JsonObject),
    Array(JsonArray),
    True,
    False,
    Null,
}
impl JsonValue {
    fn is_expected_variant(self, rhs: &Self) -> Option<Self> {
        if self.eq(rhs) {
            Some(self)
        } else {
            None
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct JsonArray(Vec<JsonValue>);
impl std::ops::Deref for JsonArray {
    type Target = Vec<JsonValue>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl std::ops::DerefMut for JsonArray {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct JsonPair {
    key: String,
    value: JsonValue,
}
#[derive(Debug, Clone, PartialEq)]
pub struct JsonObject(Vec<JsonPair>);
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ParserError {
    UnexpectedEOFError,
    InvaildJsonValueError,
    UnexpectedTokenError,
}
impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            &ParserError::UnexpectedEOFError => write!(f, "ParserUnexpectedEOFError"),
            &ParserError::InvaildJsonValueError => write!(f, "ParserInvaildJsonValueError"),
            &ParserError::UnexpectedTokenError => write!(f, "ParserUnexpectedTokenError"),
        }
    }
}
impl std::error::Error for ParserError {}
pub fn parse_json(input: Vec<JSONTokens>) -> Result<Json, ParserError> {
    Ok(Json {
        value: parse_json_value(&mut input.into_iter().peekable())?,
    })
}
fn parse_json_value(input: &mut Peekable<IntoIter<JSONTokens>>) -> Result<JsonValue, ParserError> {
    let val = input.next().ok_or(ParserError::UnexpectedEOFError)?;
    match val {
        JSONTokens::OpenBrace => {
            let obj = parse_json_object(input)?;
            Ok(JsonValue::Object(obj))
        }
        JSONTokens::Number(val) => Ok(JsonValue::Number(val)),
        JSONTokens::String(json_str) => Ok(JsonValue::String(json_str)),
        JSONTokens::OpenBracket => {
            let arr = parse_json_array(input)?;
            Ok(JsonValue::Array(arr))
        }
        JSONTokens::True => Ok(JsonValue::True),
        JSONTokens::False => Ok(JsonValue::False),
        JSONTokens::Null => Ok(JsonValue::Null),
        _ => Err(ParserError::UnexpectedTokenError),
    }
}
fn parse_json_object(input: &mut Peekable<IntoIter<JSONTokens>>) -> Result<JsonObject, ParserError> {
    // check if it is an empty object
    match input.next_if(|it| {
        println!("{:?} {:?} {} ",it,&JSONTokens::CloseBrace,it == &JSONTokens::CloseBrace);
        it == &JSONTokens::CloseBrace
    }) {
        Some(_) => return Ok(JsonObject(vec![])),
        None => {}
    };

    // obviously it is not.
    let mut pairs = vec![];
    parse_json_pair(input)?.kotlin_also_by_ref(|it| {
        pairs.push(it.clone());
    });
    while let Some(token) = input.next_if(|it| it != &JSONTokens::CloseBrace) {
        if token != JSONTokens::Comma {
            return Err(ParserError::UnexpectedTokenError);
        }
        let pair = parse_json_pair(input)?;
        pairs.push(pair);
    }
    input.next().ok_or(ParserError::UnexpectedEOFError)?;
    Ok(JsonObject(pairs))
}
fn parse_json_array(input: &mut Peekable<IntoIter<JSONTokens>>) -> Result<JsonArray, ParserError> {
    // check if it is an empty object
    match input
        .next_if(|it| it == &JSONTokens::CloseBracket)
    {
        Some(_) => return Ok(JsonArray(vec![])),
        None => {}
    };

    // obviously it is not.
    let mut values = vec![];
    parse_json_value(input)?.kotlin_also_by_ref(|it| {
        values.push(it.clone());
    });
    while let Some(token) = input
        .next_if(|it| it != &JSONTokens::CloseBracket)
    {
        if token != JSONTokens::Comma {
            return Err(ParserError::UnexpectedTokenError);
        }
        let value = parse_json_value(input)?;
        values.push(value);
    }
    input.next().ok_or(ParserError::UnexpectedEOFError)?;
    Ok(JsonArray(values))
}
fn parse_json_pair(input: &mut Peekable<IntoIter<JSONTokens>>) -> Result<JsonPair, ParserError> {
    Ok(JsonPair {
        key: match input
            .next()
            .ok_or(ParserError::UnexpectedEOFError)?
        {
            JSONTokens::String(val) => {
                input.next_if(|x| x == &JSONTokens::Colon)
                    .ok_or(ParserError::UnexpectedTokenError)?;
                val
            }
            x => {
                println!("{:#?}",x);
                return Err(ParserError::UnexpectedTokenError)
            },
        },
        value: parse_json_value(input)?,
    })
}
