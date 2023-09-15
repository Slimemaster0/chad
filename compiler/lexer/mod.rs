mod tokenizer;
mod imp;

use crate::lexer::imp::lexer_wrapped;
use crate::ast::VarInput;
use crate::{ Arguments, Type, Int, Float };
use std::fmt;

enum State {
    Default,
    VarDecl
}

#[derive(PartialEq, Debug)]
pub enum Token {
    Modifier(Modifier),
    VarDecl(Type),
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

/// Modifiers for variables eg. "mut" and "long"
#[derive(PartialEq, Debug)]
pub enum Modifier {
    Pub,
    Mut,
    Long,
    Short,
    Signed,
    Unsigned
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

impl fmt::Display for Modifier {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{:?}", self)
    }
}
