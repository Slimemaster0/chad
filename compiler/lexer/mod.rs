use crate::format::*;

pub enum Token {
    Pub,
    Mut,
    VarDecl(String), Name(String), ParenOpen, // (
    ParenClose, // )
    Assign, // =
    SqParenOpen, // [
    SqParenClose, // ]
    CurlyOpen, // {
    CurlyClose, // }
    Comma, // ,
    String(String),
    Number(i128),
    Float,
    Refrence, // &
    RawPtr, // *
    ModuleImport,
    Semicolon, // ;
    BinOP(char), // + - * /
}

pub fn lexer(input: &String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();

    {
        let chars: Vec<char> = input.chars().collect();

        let mut current_word: String = String::new();
        let mut is_comment = false; // comments
        let mut is_string = false; // Strings
        let mut is_escaped = false; // Escape chars in strings
        for i in 0..chars.len() {
            if is_comment {
                if chars[i] == '\n' {
                    is_comment = false;
                }
                continue;
            }

            let mut next_token: Result<Token, bool> = Result::Err(false);
            

            match chars[i] {
                '(' => {
                    next_token = Ok(Token::ParenOpen);
                }
                ')' => {
                    next_token = Ok(Token::ParenClose);
                }
                '[' => {
                    next_token = Ok(Token::SqParenOpen);
                }
                ']' => {
                    next_token = Ok(Token::SqParenClose);
                }
                '{' => {
                    next_token = Ok(Token::CurlyOpen);
                }
                '}' => {
                    next_token = Ok(Token::CurlyClose);
                }
                '&' => {
                    next_token = Ok(Token::Refrence);
                }
                ';' => {
                    next_token = Ok(Token::Semicolon);
                }
                '+' => {
                    next_token = Ok(Token::BinOP('+'));
                }
                '-' => {
                    if current_word.len() != 0 {
                        next_token = Ok(Token::BinOP('-'));
                    }
                }
                ',' => {
                    next_token = Ok(Token::Comma);
                }
                _ => {}
            }

            current_word.push(chars[i]);
        }
    }

    return tokens;
}
