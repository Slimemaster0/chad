use std::env;
use std::fs::File;
use std::io::prelude::*;
use crate::format::*;
use crate::lexer::{ lexer, Token, Type };

/// The main function exept all the imports arn't in the main.rs file.
pub fn init() {
    let mut debug_lexer = false;
    let mut files: Vec<String> = Vec::new();
    { // Argument parsing
        let args: Vec<String> = env::args().collect();
        for i in 0..args.len() {
            if args[i].len() == 0 {
                continue;
            }
            if args[i].len() == 1 {
                files.push(args[i].clone());
                continue;
            }
            if args[i].chars().nth(0).expect("no char at 0") == '-' {
                if args[i].chars().nth(1).expect("no char at 1") == '-' {
                    match args[i].as_str() {
                        "--debug-lexer" => debug_lexer = true,

                        _ => panic!("Invalid argument: {}", args[i]),
                    }
                } else {
                    let chars: Vec<char> = args[i].chars().into_iter().collect();
                    for j in 1..chars.len() {
                        match chars[j] {

                            _ => panic!("Invalid argument: -{}", chars[j]),
                        }
                    }
                }
                continue;
            }
            files.push(args[i].clone());
        }
    }

    let mut file: File = File::open(files[1].as_str()).expect("{RED}Err:{RESET_FORMAT} Cannot open file!");
    let mut content: String = String::new();
    file.read_to_string(&mut content).expect("{RED}Err:{RESET_FORMAT} Cannot read file!");

    let tokens = lexer(&content);

    // Prints each token to stdout
    if debug_lexer {
        for i in 0..tokens.len() {
            match &tokens[i] {
                Token::VarDecl => println!("VarDecl"),
                Token::Name(s) => println!("Name({s})"),
                Token::Assign => println!("Assign"),
                Token::FunCall => println!("FunCall"),
                Token::Number(n) => println!("Number({n})"),
                Token::Semicolon => println!("Semicolon"),
                Token::ParenOpen => println!("ParenOpen"),
                Token::ParenClose => println!("ParenClose"),
                Token::SqParenOpen => println!("SqParenOpen"),
                Token::SqParenClose => println!("SqParenClose"),
                Token::DataType(t) => {
                    match t {
                        Type::Int => println!("Type::Int"),
                        Type::Float => println!("Type::Float"),
                        Type::Bool => println!("Type::Bool"),
                        Type::Char => println!("Type::Char"),
                        Type::String => println!("Type::String"),
                        Type::Vec => println!("Type::Vec"),
                        Type::Struct => println!("Type::Struct"),
                        Type::Enum => println!("Type::Enum")
                    }
                }
                _ => {}
            }
        }
    }
}
