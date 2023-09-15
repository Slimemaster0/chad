// vim:fileencoding=utf-8:foldmethod=marker
mod var_input;
mod generation;

use crate::lexer::Token;
use crate::{ Type, Value };
use self::generation::generate;

use std::convert::From;

pub struct AST {
    pub bin_type: BinType,
    pub name_spaces: Vec<NameSpace>,
}

pub enum BinType {
    Excutable,
    Library
}

pub struct NameSpace {
    pub name: String,
    pub functions: Vec<Function>,
}

pub struct Function {
    pub name: String,
    pub sub_functions: Vec<Function>,
    pub is_public: bool,
    pub arguments: Vec<Type>,
    pub stmt: Vec<Statement>,
    pub VarDecls: Vec<Variable>
}

pub struct Variable {
    pub name: String,
    pub vtype: Type,
}

pub enum Statement {
    Add(i128, i128),
    VarDecl(Type, String, bool, Option<Value>),
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

/// Parses tokens and returns an AST
pub fn ast_gen(tokens: Vec<Token>, bin_type: BinType) -> AST {
    generate(tokens, bin_type)
}
