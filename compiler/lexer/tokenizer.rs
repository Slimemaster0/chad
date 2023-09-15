use crate::lexer::{ Token, Modifier };
use crate::{ Type, Int, Float };
use crate::format::*;

pub fn tokenize(word: &String) -> Result<Vec<Token>, &str> {
    if word.len() == 0 {
        return Result::Err("Input lengh is zero");
    }
    match word.as_str() {
        "int" => return Result::Ok(vec![Token::VarDecl(Type::Int)]),

        "printf" => return Result::Ok(vec![Token::Name(String::from("printf"))]),

        "mut" => return Result::Ok(vec![Token::Modifier(Modifier::Mut)]),

        _ => {}
    }
    match word.parse::<i32>() {
        Ok(i) => { return Result::Ok(vec![Token::Number(Int::I32(i))]); }
        Err(_) => {}
        
    }
    match word.parse::<i64>() {
        Ok(i) => { return Result::Ok(vec![Token::Number(Int::I64(i))]); }
        Err(_) => {}
        
    }
    match word.parse::<i128>() {
        Ok(i) => { return Result::Ok(vec![Token::Number(Int::I128(i))]); }
        Err(_) => {}
    }
    match word.parse::<u128>() {
        Ok(i) => { return Result::Ok(vec![Token::Number(Int::U128(i))]); }
        Err(_) => {}
    }
    match word.parse::<f32>() {
        Ok(x) => { return Result::Ok(vec![Token::Float(crate::Float::F32(x))]); }
        Err(_) => {}
    }
    match word.parse::<f64>() {
        Ok(x) => { return Result::Ok(vec![Token::Float(crate::Float::F64(x))]); }
        Err(_) => {}
    }
    return Ok(vec![Token::Name(word.clone())])
}
