mod tokenizer;
mod imp;

use crate::lexer::imp::lexer_wrapped;

enum State {
    Default,
    VarDecl
}

pub enum Token {
    Pub,
    Mut,
    VarDecl,
    DataType(Type),
    Name(String),
    ParenOpen, // (
    ParenClose, // )
    Assign, // =
    SqParenOpen, // [
    SqParenClose, // ]
    /* comming soon™
       CurlyOpen, // {
       CurlyClose, // }
       */
    Comma, // ,
    String(String),
    Number(i128),
    Float(f64),
    //    Refrence, // &   comming soon™
    RawPtr, // *
    ModuleImport,
    Semicolon, // ;
    BinOP(char), // + - * /
    FunCall,
}

pub enum Type {
    Int,
    Bool,
    Float,
    Char,
    String,
    Vec,
    Struct,
    Enum,
}

/// Breaks a string into tokens and verify that they conform to the syntax
pub fn lexer(input: &String) -> Vec<Token> {
    lexer_wrapped(input)
}
