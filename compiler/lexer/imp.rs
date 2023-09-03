// vim:fileencoding=utf-8:foldmethod=marker
use crate::format::*;
use crate::lexer::tokenizer::tokenize;
use crate::lexer::*;
use crate::Arguments;
use crate::string_is_numeric;

pub fn lexer_wrapped(input: &String, args: &Arguments) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();

    { // Tokenization
        let chars: Vec<char> = input.chars().collect();

        let mut current_word: String = String::new();
        let mut is_comment = false; // comments
        let mut is_string = false; // Strings
        let mut is_escaped = false; // Escape chars in strings
        let mut current_stmt: Vec<Token> = Vec::new();
        let mut char_y: usize = 1;
        let mut char_x: usize = 1;
        for i in 0..chars.len() {
            if is_comment {
                if chars[i] == '\n' {
                    is_comment = false;
                }
                continue;
            }
            parse_char(
                args,
                &chars,
                &i,
                &mut tokens,
                &mut current_stmt,
                &mut current_word,
                &mut is_comment,
                &mut is_string,
                &mut is_escaped,
                &mut char_y,
                &mut char_x,
                );
            char_x+=1;
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
           */
    }

    return tokens;
    } // }}}

fn parse_char(
    args: &Arguments,
    chars: &Vec<char>,
    i: &usize,
    tokens: &mut Vec<Token>,
    current_stmt: &mut Vec<Token>,
    current_word: &mut String,
    is_comment: &mut bool,
    is_string: &mut bool,
    is_escaped: &mut bool,
    char_y: &mut usize,
    char_x: &mut usize,
    ) {
    match chars[i.clone()] {
        '(' => {
            push_n_clear_buff(current_stmt, current_word, args);
            match &current_stmt[current_stmt.len() -1] {
                Token::Name(n) => {
                    let fn_name = n.clone();
                    current_stmt.pop();
                    current_stmt.push(Token::FunCall);
                    current_stmt.push(Token::Name(fn_name));
                }
                _ => {}
            }
            current_stmt.push(Token::ParenOpen);
            return;
        }
        ')' => {
            push_n_clear_buff(current_stmt, current_word, args);
            current_stmt.push(Token::ParenClose);
            return;
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
            push_n_clear_buff(current_stmt, current_word, args);

            // Parses the current statement
            {
                let mut skip: usize = 0;
                for j in 0..current_stmt.len() {
                    if skip > 0 {
                        skip-=1;
                        continue;
                    }

                    match current_stmt[j] {
                        _ => {}
                    }
                }
            }

            current_stmt.push(Token::Semicolon);
            tokens.append(current_stmt);

            let mut empty_vec: Vec<Token> = Vec::new();
            *current_stmt = Vec::new();
            return;
        }
        // {{{ Binary operators
        '=' =>  {
            current_stmt.push(Token::Assign);
            return;
        }
        '+' => {
            push_n_clear_buff(current_stmt, current_word, args);

            current_stmt.push(Token::BinOP('+'));
            return;
        }
        '-' => {
            push_n_clear_buff(current_stmt, current_word, args);

            current_stmt.push(Token::BinOP('-'));
            return;
        }
        // }}}
        ',' => {
            push_n_clear_buff(current_stmt, current_word, args);
            current_stmt.push(Token::Comma);
            return;
        }
        ' ' => { push_n_clear_buff(current_stmt, current_word, args); return; }
        '\n' => { push_n_clear_buff(current_stmt, current_word, args); *char_y+=1; *char_x = 0 as usize; return; }
        _ => {}
    } // }}}

current_word.push(chars[*i]);
}

/// Parses the text buffer, appends the result to the tokens vector then clears the text buffer
fn push_n_clear_buff(tokens: &mut Vec<Token>, txt_buff: &mut String, args: &Arguments) {
    if args.debug_lexer { println!("txt_buff: {txt_buff}") };
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
    *txt_buff = String::new();
} // }}}
