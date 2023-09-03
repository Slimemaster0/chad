use std::env;
use std::fs::File;
use std::io::prelude::*;
use crate::format::*;
use crate::lexer::{ lexer, Token };
use crate::{ Arguments, Type };

/// The main function exept all the imports arn't in the main.rs file.
pub fn init() {
    let mut args: Arguments = Arguments::new();
    { // Argument parsing
        let args_raw: Vec<String> = env::args().collect();
        for i in 0..args_raw.len() {
            if args_raw[i].len() == 0 {
                continue;
            }
            if args_raw[i].len() == 1 {
                args.files.push(args_raw[i].clone());
                continue;
            }
            if args_raw[i].chars().nth(0).expect("no char at 0") == '-' {
                if args_raw[i].chars().nth(1).expect("no char at 1") == '-' {
                    match args_raw[i].as_str() {
                        "--debug-lexer" => args.debug_lexer = true,

                        _ => panic!("Invalid argument: {}", args_raw[i]),
                    }
                } else {
                    let chars: Vec<char> = args_raw[i].chars().into_iter().collect();
                    for j in 1..chars.len() {
                        match chars[j] {

                            _ => panic!("Invalid argument: -{}", chars[j]),
                        }
                    }
                }
                continue;
            }
            args.files.push(args_raw[i].clone());
        }
    }

    let mut file: File = File::open(args.files[1].as_str()).expect("{RED}Err:{RESET_FORMAT} Cannot open file!");
    let mut content: String = String::new();
    file.read_to_string(&mut content).expect("{RED}Err:{RESET_FORMAT} Cannot read file!");

    let tokens = lexer(&content, &args);

    // Prints each token to stdout
    if args.debug_lexer {
        for i in 0..tokens.len() {
            println!("{}", tokens[i]);
        }
    }
}
