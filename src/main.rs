use std::collections::HashMap;
use easyjson_rs::{lexer::Lexer, parser::parse_json, useful_kt_extensions::KotlinScopeFunction};

fn main() -> Result<(),Box<dyn std::error::Error>> {
    let simple = "{\"name\":\"sjy\",\"age:\":20}";
    let json_input = r#"
        {
            "name":"sjy",
            "age":22,
            "isMale":true,
            "girlFriend":null,
            "friends":[
                {
                    "name":"dla",
                    "age":20,
                    "isMale":true,
                    "friends":1000,
                    "isWHUStudent":true
                },
                {
                    "name":"lhr",
                    "age":21,
                    "isMale":true,
                    "friends":[{
                        "name":"dzy",
                        "age":21,
                        "isMale":true,
                        "isWHUStudent":true,
                        "friends":{
                            "name":"lyx",
                            "age":21,
                            "isMale":true
                        }
                    }]
                },{
                    "name":"dzy",
                    "age":21,
                    "isMale":true,
                    "isWHUStudent":true,
                    "friends":{
                        "name":"lyx",
                        "age":21,
                        "isMale":true
                    }
                }
            ]
        }
    "#;
    println!("{:#?}",parse_json(Lexer::new().get_json_tokens(json_input)?)?);
    Ok(())
}