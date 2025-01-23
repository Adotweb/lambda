mod lexer;
mod parser;

use lexer::*;
use parser::*;

fn main() {
    let string = "#hello #new #directive";

    let token_list = lex(string);

    let ast = parse(&token_list);

    println!("{:#?}", ast);
}
