mod lexer;
mod parser;
mod eval;

use lexer::*;
use parser::*;
use eval::*;

fn main() {
    let string = "
        #hello;

        #print_e ((y -> z -> y y) (x -> x));     
    
        ";

    let token_list = lex(string);

    //println!("{:#?}", token_list);

    let ast = parse(&token_list);

    //println!("{}", ast.get_display());

    let root = Environment::new();

    evaluate(ast, root.clone());
}
