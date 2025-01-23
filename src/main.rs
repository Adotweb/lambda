mod eval;
mod lexer;
mod parser;

use eval::*;
use lexer::*;
use parser::*;

use std::collections::HashMap;

fn main() {
    let string = "

        #zero (f -> x -> x);

        #succ (n -> f -> x -> f (n f x));

        #plus (n -> m -> m succ n);

        #one (#succ #zero);

        #two (#succ #one);

        #print_u (#one #succ #one);
        ";

    let token_list = lex(string);

    //println!("{:#?}", token_list);

    let mut vc = VarCounter(HashMap::new());

    let ast = parse(&token_list, &mut vc);

    //println!("{}", ast.get_display());

    let root = Environment::new();

    evaluate(ast, root.clone());
}
