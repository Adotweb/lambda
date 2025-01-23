use crate::TokenType;

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Expression {
    Global {
        list: Vec<Expression>,
    },

    App {
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Grp {
        inner: Box<Expression>,
    },
    Directive {
        name: String,
    },
    Def {
        argument: String,
        body: Box<Expression>,
    },
    Number(usize),
    Unit(String),
    None,
}

impl Expression {
    fn chain_app(&self, left: Expression) -> Self {
        if let Expression::None = left {
            self.clone()
        } else {
            Self::App {
                left: Box::new(left),
                right: Box::new(self.clone()),
            }
        }
    }

    pub fn get_display(&self) -> String {
        match self {
            Self::Global { list } => {
                format!(
                    "{}",
                    list.iter()
                        .map(|x| { x.get_display() })
                        .collect::<Vec<String>>()
                        .join(";\n")
                        + ";"
                )
            }
            Self::App { left, right } => {
                format!("{} {}", left.get_display(), right.get_display())
            }
            Self::Grp { inner } => {
                format!("({})", inner.get_display())
            }
            Self::Def { argument, body } => {
                format!("{} -> {}", argument, body.get_display())
            }
            Self::Unit(unit) => unit.to_string(),
            Self::None => "".to_string(),
            Self::Number(num) => num.to_string(),
            Self::Directive { name } => {
                format!("#{}", name)
            }

            _ => String::new(),
        }
    }
}

//this thing keeps track and makes alpha conversions whenever it needs (more than one f is used in
//a function)
pub struct VarCounter(pub HashMap<String, usize>);



pub fn parse(tokens: &Vec<TokenType>, vc : &mut VarCounter) -> Expression {
    let mut list: Vec<Expression> = vec![];

    let mut current_index = 0;

    let mut expr_num = 0;

    while let Some(token) = tokens.get(current_index) {
        let expr = app(tokens, &mut current_index, vc);

        if let Expression::None = expr {
            continue;
        }

        list.push(expr);
    }

    Expression::Global { list }
}

pub fn app(tokens: &Vec<TokenType>, current_index: &mut usize, vc : &mut VarCounter) -> Expression {
    let mut left_most = Expression::None;

    //this will *always* consume a token
    //*current_index += 1;

    while let Some(current_token) = tokens.get(*current_index) {
        match current_token {
            TokenType::NUMBER(num) => {
                *current_index += 1;
                left_most = Expression::Number(*num).chain_app(left_most);
            }
            TokenType::HASH => {
                *current_index += 1;

                if let TokenType::IDENTIFIER(id) = tokens.get(*current_index).unwrap() {
                    *current_index += 1;

                    let hash = Expression::Directive {
                        name: id.to_string(),
                    };
                    left_most = hash.chain_app(left_most);
                } else {
                    panic!("after hash a directive Identifier is needed!")
                }
            }
            TokenType::IDENTIFIER(id) => {
                *current_index += 1;

                if let Some(current_token) = tokens.get(*current_index) {
                    if let TokenType::ARROW = current_token {
                        *current_index += 1;

                        let body = app(tokens, current_index, vc);


                        let func = Expression::Def {
                            argument: id.to_string(),
                            body: Box::new(body),
                        };

                        left_most = func.chain_app(left_most);
                        continue;
                    }

                    let this_unit = Expression::Unit(id.to_string());

                    left_most = this_unit.chain_app(left_most);
                }
            }
            TokenType::LPAREN => {
                *current_index += 1;

                if let TokenType::RPAREN = tokens.get(*current_index).unwrap() {
                    panic!("groups cannot be empty!")
                }

                let grp = Expression::Grp {
                    inner: Box::new(app(tokens, current_index, vc)),
                };

                *current_index += 1;

                left_most = grp.chain_app(left_most);
            }
            TokenType::RPAREN => {
                return left_most;
            }
            TokenType::SEMICOLON => {
                *current_index += 1;
                return left_most;
            }

            _ => panic!("unexpected token {:?}", current_token),
        }
    }
    left_most
}
