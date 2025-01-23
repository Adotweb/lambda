use crate::TokenType;

#[derive(Debug, Clone)]
pub enum Expression{
    Global{
        list : Vec<Expression>
    },
   
    App{
        left : Box<Expression>,
        right : Box<Expression>
    },
    Grp{
        inner : Box<Expression>
    },
    Directive{
        name : String
    },
    Def{
        argumnet : String,
        body : Box<Expression>
    },
    Number(usize),
    Unit(String),
    None

}

impl Expression{
    fn chain_app(&self, left : Expression) -> Self{
        if let Expression::None = left {
            self.clone()
        } else {
            Self::App {
                left : Box::new(left),
                right : Box::new(self.clone())
            } 
        }
    }
}

pub fn parse(tokens : &Vec<TokenType>) -> Expression{
    
    let mut list : Vec<Expression> = vec![];
  
    let mut current_index = 0;

    while let Some(token) = tokens.get(current_index){
        let expr = app(tokens, &mut current_index);
        list.push(expr);
        current_index += 1;
    }
    
    Expression::Global{
        list
    }
}

pub fn app(tokens : &Vec<TokenType>, current_index : &mut usize) -> Expression{

    let mut left_most = Expression::None;

    //this will *always* consume a token 
    //*current_index += 1;

    while let Some(current_token) = tokens.get(*current_index){

        

        match current_token{
            TokenType::HASH => {
                *current_index += 1;
                
                if let TokenType::IDENTIFIER(id) = tokens.get(*current_index).unwrap(){
                    *current_index += 1;
                    let hash = Expression::Directive{name : id.to_string()};
                    left_most = hash.chain_app(left_most);
                } else {
                    panic!("after hash a directive Identifier is needed!")
                }

            }
            TokenType::IDENTIFIER(id) => {
                *current_index += 1;

                if let TokenType::ARROW = tokens.get(*current_index).unwrap(){
                    *current_index += 1;

                    let func = Expression::Def{
                        argumnet : id.to_string(), 
                        body : Box::new(app(tokens, current_index))
                    };

                    left_most = func.chain_app(left_most);
                }
            }
            TokenType::LPAREN => {
                *current_index += 1;
                    
                if let TokenType::RPAREN = tokens.get(*current_index).unwrap(){
                    panic!("groups cannot be empty!")
                }


                let grp = Expression::Grp{
                    inner : Box::new(app(tokens, current_index))
                };
                
                if let TokenType::RPAREN = tokens.get(*current_index).unwrap(){
                    left_most = grp.chain_app(left_most);
                }else {
                    panic!("expected right parenthesis here to close the group!")
                }
            },
            TokenType::SEMICOLON => {
                *current_index += 1;
               
            
                return left_most;
            }

            _ => panic!("unexpected token {:?}", current_token)
        } 
       

    };
    left_most

}

