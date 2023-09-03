mod lexer;
mod format;
mod astgen;
mod init;

use crate::init::init;
use std::fmt;

pub struct Arguments {
    pub debug_lexer: bool,
    pub files: Vec<String>,
}

impl Arguments {
    pub fn new() -> Arguments {
        return Arguments {
            debug_lexer: false,
            files: Vec::new(),
        };
    }
}

fn main() {
    init();    
}

pub fn string_is_numeric(s: &str) -> bool {
    for c in s.chars() {
        if !c.is_numeric() { return false; }
    }
    return true;
}

#[derive(PartialEq, Debug)]
pub enum Value {
    Int(Int),
    Bool(bool),
    Float(f64),
    Char(char),
    String(String),
    Vec(Vec<Value>),
    Struct(String, Vec<Value>, Vec<String>),
    Enum(String, usize, Vec<Value>),
}

#[derive(PartialEq, Debug, Clone)]
pub enum Type {
    Int,
    Bool,
    Float,
    Char,
    String,
    Vec,
    Struct(String),
    Enum(String),
    Void
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Int {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    USize(usize),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    ISize(isize),
}

#[derive(PartialEq, Debug)]
pub enum Float {
    F32(f32),
    F64(f64)
}

impl fmt::Display for Type {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{:?}", self)
    }
}

impl fmt::Display for Value {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{:?}", self)
    }
}

impl fmt::Display for Int {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{:?}", self)
    }
}

impl fmt::Display for Float {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{:?}", self)
    }
}
