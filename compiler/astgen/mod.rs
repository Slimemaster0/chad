// vim:fileencoding=utf-8:foldmethod=marker
mod var_input;
mod imp;

use crate::lexer::Token;
use crate::{ Type, Value };
use crate::astgen::imp::generate_AST_wrapped;

use std::convert::From;

pub struct AST {
    pub bin_type: BinType,
    pub files: Vec<SrcFile>,
}

pub enum BinType {
    Excutable,
    Library
}

pub struct SrcFile {
    pub name: String,
    pub functions: Vec<Function>,
}

pub struct Function {
    pub name: String,
    pub sub_functions: Vec<Function>,
    pub is_public: bool,
    pub arguments: Vec<Type>,
    pub instructions: Vec<Instruction>,
    pub VarDecls: Vec<Variable>
}

pub struct Variable {
    pub name: String,
    pub vtype: Type,
}

pub enum Instruction {
    Add(i128, i128),
}

pub enum VarInput {
    Literal(Value),
    Variable(String),
    Function(Type) // Not implemented yet
}

pub struct BinOP {
    pub operator: char,
    pub left: VarInput<>,
    pub right: VarInput<>
}

/// Parses tokens and returns an AST wrapped
pub fn generate_AST(tokens: Vec<Token>, bin_type: BinType) -> AST {
    generate_AST_wrapped(tokens, bin_type)
}

