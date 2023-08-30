use crate::lexer::{ Token, Type };
use crate::format::*;

use crate::astgen::*;

/// Parses tokens and returns an AST wrapped
pub fn generate_AST_wrapped(tokens: Vec<Token>, bin_type: BinType) -> AST {
    let mut ast = AST {
        bin_type: bin_type,
        files: vec![
            SrcFile { name: String::from("main.chad"),
            functions: vec![
                Function {
                    name: String::from("main"),
                    sub_functions: Vec::new(),
                    is_public: false,
                    arguments: Vec::new(),
                    instructions: Vec::new(),
                    VarDecls: Vec::new(),
                }
            ]}]
    };

    { // Parsing
        let mut function_ception: Vec<usize> = Vec::new();
        let mut context = String::new();
        for mut i in 0..tokens.len() {
            match tokens[i] {
                Token::Number(int) => {}
                Token::BinOP(op) => {
                    let i1: VarInput<i128>;
                    let i2: VarInput<i128>;
                    
                    match tokens[i-1] {
                        Token::Number(int) => {
                            i1 = VarInput::Literal(int);
                        }
                        _ => panic!("{RED}Err:{RESET_FORMAT} binary operators must be give to numbers eg: {BOLD}69 {op} 420;{RESET_FORMAT}"),
                    }

                    match tokens[i+1] {
                        Token::Number(int) => {
                            i2 = VarInput::Literal(int);
                        }
                        _ => panic!("{RED}Err:{RESET_FORMAT} binary operators must be give to numbers eg: {BOLD}69 {op} 420;{RESET_FORMAT}"),
                    }

                    match op {
                        '+' => {
                            
                        }
                        _ => panic!("BinOP char must be: '+', '-', '*', '/'")
                    }
                }
                _ => println!("{RED}Err:{RESET_FORMAT} Not inplemented!"),
            }
        }
    }

    return ast;
}
