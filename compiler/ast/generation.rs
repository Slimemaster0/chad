// vim:fileencoding=utf-8:foldmethod=marker
use crate::lexer::{ Token, Modifier };
use crate::format::*;

use crate::ast::*;


/// generates an AST from a Vec<Token>
pub fn generate(tokens: Vec<Token>, bin_type: BinType) -> AST {
    // {{{ Structs and enums
    struct Modifiers {
        pub mut_: bool,
        pub long: u8,
        pub short: bool,
        pub signed: bool,
        pub unsigned: bool,
    }

    impl Modifiers {
        fn new() -> Modifiers {
            Modifiers { mut_: false, long: 0, short: false, signed: false, unsigned: false}
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
        name_space: &mut AST,
        current_fn: &usize
        ) {

        /// A function for finding the function being appended to.
        /// The &Vec<usize> named i cannot be empty.
        fn fn_finder<'a>(functions: &'a mut Vec<Function>, i: &Vec<usize>) -> &'a mut Function { // {{{
            if i.len() == 0 {
                panic!("{RED}Err:{RESET_FORMAT} fn_finder i should not be empty, plz report this is a bug in the compiler. UwU")
            }
            if i.len() == 1 {
                return &mut functions[i[0]];
            }
            let mut j = i.clone();
            let mut ret = fn_finder(&mut functions[i[0]].sub_functions, &j);
            ret
        } // }}}

        fn name_extraction(input: &Token) -> String {
            match input {
                Token::Name(s) => return *s,
                _ => panic!("Expected a name")
            }
        }

        match tokens[*i] {
            Token::Modifier(m) => {
                match m {
                    Modifier::Mut => modifiers.mut_ = true,
                    Modifier::Long => modifiers.long += 1,
                    Modifier::Short => modifiers.short = true,
                    Modifier::Signed => modifiers.signed = true,
                    Modifier::Unsigned => modifiers.unsigned = true,
                }
            }

            Token::VarDecl(mut t) => {
                match t {
                    Type::Struct(_) => {
                        t = Type::Struct(name_extraction(&tokens[*i+1]));
                        *i+=1;
                    }

                    Type::Enum(_) => {
                        panic!("NOT implemented!");
                    }

                    _ => {}
                }
                let name: String = name_extraction(&tokens[*i+1]);
                
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
    functions: vec![
        Function {
            name: String::from("main"),
            sub_functions: Vec::new(),
            arguments: Vec::new(),
            stmt: Vec::new(),
            VarDecls: Vec::new(),
        }
    ]
};

{ // Parse tokens
    let mut current_fn: usize = 0;
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
            &mut ast,
            &current_fn
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
