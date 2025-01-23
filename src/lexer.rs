#[derive(Debug, Clone)]
pub enum TokenType{ 
    LPAREN, 
    RPAREN, 
    DOT,
    HASH,
    
    ARROW,

    EQ,

    IDENTIFIER(String),

    NUMBER(usize),
    SEMICOLON,

    EOF
}

pub fn iterate_whitespace(chars: Vec<String>, current_index : &mut usize){

    while let Some(c) = chars.get(*current_index){
        if c != " "{
            *current_index -= 1;
            return 
        }
        *current_index += 1;
    }
    
}

pub fn iterate_id(id_match : Vec<String>, chars: Vec<String>, current_index : &mut usize, tokens : &mut Vec<TokenType>){
  
    let mut identifier_string = String::new();

    while let Some(c) = chars.get(*current_index){
        if !id_match.contains(c){
            tokens.push(TokenType::IDENTIFIER(identifier_string.clone()));
            return
        }
        identifier_string += c;
        *current_index += 1;
    }

}


pub fn iterate_num(num_match : Vec<String>, chars: Vec<String>, current_index : &mut usize, tokens : &mut Vec<TokenType>){
  
    let mut number = String::new();

    while let Some(c) = chars.get(*current_index){
        if !num_match.contains(c){
            tokens.push(TokenType::NUMBER(number.clone().parse::<usize>().unwrap()));
            return  
        }
        number += c;
        *current_index += 1;
    }

}

pub fn lex<'a>(text : &'a str) -> Vec<TokenType>{

    let mut tokens : Vec<TokenType> = Vec::new();

    let mut current_index = 0;
    let mut lines = 1;
    let mut chars : Vec<String> = text.chars().map(|x|x.to_string()).filter(|x|{
        x != ""
    }).into_iter().collect();

    //we push one white space character to the very end so that we dont have to worry about not
    //finishing any ids or nums
    chars.push(" ".to_string());

    let id_match : Vec<String> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_".chars()
        .map(|x|x.to_string())
        .collect();

    let num_match : Vec<String> = "0123456789".chars()
        .map(|x|x.to_string())
        .collect();

    while let Some(current_char) = chars.get(current_index){

        match current_char.as_str(){
            "#" => tokens.push(TokenType::HASH),
            "(" => tokens.push(TokenType::LPAREN),
            ")" => tokens.push(TokenType::RPAREN),
            "." => tokens.push(TokenType::DOT),
            "=" => tokens.push(TokenType::EQ),
            ";" => tokens.push(TokenType::SEMICOLON),
            "-" => {
                if let Some(s) = chars.get(current_index + 1){
                    if s == ">"{
                        tokens.push(TokenType::ARROW)
                    } else {
                        panic!("expected > after - on line {lines}")
                    }
                }
            }
            " " => iterate_whitespace(chars.clone(), &mut current_index),
            "\n" => {
                lines += 1;
            }
            _ =>{
                if num_match.contains(current_char){
                    iterate_num(num_match.clone(), chars.clone(), &mut current_index, &mut tokens);
                } 

                if id_match.contains(current_char){
                    iterate_id(id_match.clone(), chars.clone(), &mut current_index, &mut tokens);
                }
            }
        }
     

         current_index += 1;
    }
    
    //tokens.push(TokenType::EOF);
        
    tokens
}
