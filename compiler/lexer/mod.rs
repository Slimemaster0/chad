mod tokenizer;
mod imp;

use crate::lexer::imp::lexer_wrapped;
use crate::astgen::VarInput;
use crate::{ Arguments, Type, Int, Float };
use std::fmt;

enum State {
    Default,
    VarDecl
}

#[derive(PartialEq, Debug)]
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
    Number(Int),
    Float(Float),
    //    Refrence, // &   comming soon™
    RawPtr, // *
    ModuleImport,
    Semicolon, // ;
    BinOP(char), // + - * /
    FunCall,
}

/// Breaks a string into tokens and verify that they conform to the syntax
pub fn lexer(input: &String, args: &Arguments) -> Vec<Token> {
    lexer_wrapped(input, args)
}

impl fmt::Display for Token {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{:?}", self)
    }
}
