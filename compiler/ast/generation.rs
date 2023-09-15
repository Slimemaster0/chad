// vim:fileencoding=utf-8:foldmethod=marker
use crate::lexer::{ Token, Modifier };
use crate::format::*;

use crate::ast::*;


/// 
pub fn generate(tokens: Vec<Token>, bin_type: BinType) -> AST {
    // {{{ Structs and enums
    struct Modifiers {
        pub pub_: bool,
        pub mut_: bool,
        pub long: u8,
        pub short: bool,
        pub signed: bool,
        pub unsigned: bool,
    }

    impl Modifiers {
        fn new() -> Modifiers {
            Modifiers { pub_: false, mut_: false, long: 0, short: false, signed: false, unsigned: false}
        }
    }
    // }}}

    /// Parse a token
    fn parse( // {{{
        tokens: &Vec<Token>,
        i: &mut usize,
        context: &mut String,
        current_stmt: &mut usize,
        modifiers: &mut Modifiers,
        function_ception: &mut Vec<usize>,
        name_space: &mut NameSpace,
        ) {

        fn fn_finder(name_space: &NameSpace, ception: &Vec<usize>) {
            
        }

        match tokens[*i] {
            Token::Modifier(m) => {
                match m {
                    Modifier::Pub => modifiers.pub_ = true,
                    Modifier::Mut => modifiers.mut_ = true,
                    Modifier::Long => modifiers.long += 1,
                    Modifier::Short => modifiers.short = true,
                    Modifier::Signed => modifiers.signed = true,
                    Modifier::Unsigned => modifiers.unsigned = true,
                }
            }

            Token::VarDecl(t) => {
                let mut j: usize = *i + 1;
                let mut name = String::new();
                loop {
                    match tokens[j] {
                        Token::Name(s) => {
                            name = s;
                        }

                        Token::Assign => {

                        }

                        Token::Semicolon => {
                        }
                        _ => panic!("{RED}Syntax-error:{RESET_FORMAT} Please specify the name of the variable"),
                    }
                    j+=1;
                }
                *modifiers = Modifiers::new();
            }

            /*
               Token::Number(int) => {}
               Token::BinOP(op) => {
               let i1: VarInput;
               let i2: VarInput;

               match tokens[i-1] {
               Token::Number(int) => {

               }
               _ => panic!("{RED}Err:{RESET_FORMAT} binary operators must be give to numbers eg: {BOLD}69 {op} 420;{RESET_FORMAT}"),
               }

               match tokens[i+1] {
               Token::Number(int) => {
            //i2 = VarInput::Literal(int);
            }
            _ => panic!("{RED}Err:{RESET_FORMAT} binary operators must be give to numbers eg: {BOLD}69 {op} 420;{RESET_FORMAT}"),
            }

            match op {
            '+' => {

            }
            _ => panic!("BinOP char must be: '+', '-', '*', '/'")
            }
            }
            */
            _ => println!("{RED}Err:{RESET_FORMAT} Not implemented!"),
        }

    } // }}}

    // {{{ The actual function
    let mut ast = AST {
        bin_type: bin_type,
        name_spaces: vec![
            NameSpace {
                name: String::from("main"),
                functions: vec![
                    Function {
                        name: String::from("main"),
                        sub_functions: Vec::new(),
                        is_public: false,
                        arguments: Vec::new(),
                        stmt: Vec::new(),
                        VarDecls: Vec::new(),
                    }
                ]}]
    };

    { // Parse tokens
        let mut current_ns: usize = 0;
        let mut function_ception: Vec<usize> = Vec::new();
        let mut context = String::new();
        let mut current_stmt: usize = 0;
        let mut modifiers = Modifiers::new();
        for mut i in 0..tokens.len() {
            parse(
                &tokens,
                &mut i,
                &mut context,
                &mut current_stmt,
                &mut modifiers,
                &mut function_ception,
                &mut ast.name_spaces[current_ns]
            )
        }
    }

    return ast;
    // }}}
}

fn add_stmt(ast: &mut AST, function_ception: &Vec<usize>, input: Statement) {
    if function_ception.len() < 2 {
        //
    }
}
