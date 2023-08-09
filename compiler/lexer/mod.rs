use crate::format::*;

pub enum Token {
    VarDecl(String, VarMetadata),
    Name(String),
    ParenOpen, // (
    ParenClose, // )
    Assign, // =
    SqParenOpen, // [
    SqParenClose, // ]
    CurlyOpen, // {
    CurlyClose, // }
    Comma, // ,
    String(String),
    Number,
    Float,
    FunCall(String),
    VarCall(String),
    ModuleImport(String),
    Semicolon, // ;
}

pub struct VarMetadata {
    pub is_public: bool,
    pub is_func: bool
}

pub fn lexer(input: &String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();

    {
        let chars: Vec<char> = input.chars().collect();

        let mut current_word: String = String::new();
        for i in 0..chars.len() {
            if chars[i] != ' '
                && chars[i] != '('
                && chars[i] != ')'
                && chars[i] != '['
                && chars[i] != ']'
                && chars[i] != '{'
                && chars[i] != '}'
                && chars[i] != ';' 
                && chars[i] != '\n' {
                current_word.push(chars[i]);
            } else {
                match chars[i] {
                    ' ' => { tokens.push(tokenizer(current_word)); current_word = String::new(), }
                    _ => panic!("{RED}ERR:{RESET_FORMAT} The end char: '{BOLD}{}{RESET_FORMAT}' is not implemented!", chars[i]),
                }
            }
        }
    }

    return tokens;
}

pub fn tokenizer(input: &String) -> Token {
    
}
