mod tokenizer;

use crate::format::*;
use crate::lexer::tokenizer::tokenize;
use crate::ext::string_is_numeric;

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
    let mut tokens: Vec<Token> = Vec::new();

    { // Tokenization
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


            // }}}

        match chars[i] { //
            '(' => {
                push_n_clear_buff(&mut tokens, &mut current_word);
                match &tokens[tokens.len() -1] {
                    Token::Name(n) => {
                        let fn_name = n.clone();
                        tokens.pop();
                        tokens.push(Token::FunCall);
                        tokens.push(Token::Name(fn_name));
                    }
                    _ => {}
                }
                tokens.push(Token::ParenOpen);
                continue;
            }
            ')' => {
                push_n_clear_buff(&mut tokens, &mut current_word);
                tokens.push(Token::ParenClose);
                continue;
            }
/* Comming soon™
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
*/
            ';' => {
                push_n_clear_buff(&mut tokens, &mut current_word);
                tokens.push(Token::Semicolon);
                continue;
            }
            '=' =>  {
                tokens.push(Token::Assign);
                continue;
            }
            '+' => {
                if current_word.len() != 0 && string_is_numeric(&current_word) {
                    tokens.push(Token::BinOP('+'));
                    continue;
                }
            }
            '-' => {
                if current_word.len() != 0 && string_is_numeric(&current_word) {
                    tokens.push(Token::BinOP('-'));
                    continue;
                }
            }
            ',' => {
                push_n_clear_buff(&mut tokens, &mut current_word);
                tokens.push(Token::Comma);
                continue;
            }
            ' ' => { push_n_clear_buff(&mut tokens, &mut current_word); continue; }
            '\n' => { push_n_clear_buff(&mut tokens, &mut current_word); continue; }
            _ => {}
        } // }}}

        current_word.push(chars[i]);
        }
    }

    { // Verification Comming soon™
      /*
        let mut state: State = State::Default;
        for i in 0..tokens.len() {
            match state {
                State::Default => {
                    match tokens[i] {
                        Token::VarDecl => state = State::VarDecl,
                        
                        _ => panic!("{RED}Err:{RESET_FORMAT} Not all tokens have been inplemented yet.")
                    }
                }
                State::VarDecl => {
                    match Token::VarDecl {
                        
                    }
                }
            }
        }
      */
    }

    return tokens;
} // }}}

/// Parses the text buffer, appends the result to the tokens vector then clears the text buffer
fn push_n_clear_buff(tokens: &mut Vec<Token>, txt_buff: &mut String) {
    match tokenize(&txt_buff).as_mut() {
        Ok(new_tokens) => {
            if new_tokens.len() != 0 {
                tokens.append(new_tokens);
            }
        }
        Err(e) => if e.clone() != "Input lengh is zero" {
            panic!("{e}");
        },
    }
    while txt_buff.len() != 0 {
        txt_buff.pop();
    }
} // }}}
