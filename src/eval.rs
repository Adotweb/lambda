use crate::Expression;

use std::rc::Rc;
use std::cell::RefCell;

use std::collections::HashMap;

#[derive(Default)]
pub struct Environment{
    values : HashMap<String, Expression>,
    enclosing : Option<Rc<RefCell<Environment>>>
}


impl Environment{
    

    pub fn new() -> Rc<RefCell<Self>>{
        return Rc::new(RefCell::new(Self::default()))
    }

    fn link(&self, enclosing: Environment) -> Rc<RefCell<Self>>{
        let mut new_env = Environment::default();

        new_env.enclosing = Some(Rc::new(RefCell::new(enclosing)));

        Rc::new(RefCell::new(new_env))
    }

    fn set(&mut self, key : String, new_value : Expression) -> Result<(), ()>{
        
        if let Some(value) = self.values.get_mut(&key){
            *value = new_value;
            return Ok(())
        } else if let Some(enclosing) = &self.enclosing {
            let value_exists = enclosing.borrow_mut().set(key.clone(), new_value.clone());
 
            if value_exists.is_ok(){
                return value_exists; 
            }else {

                self.values.insert(key.clone(), new_value.clone());
                return Ok(());
            }
        } else {

            return Err(());
        }
    }

    fn get(&self, key : String) -> Expression{
        if let Some(value) = self.values.get(&key) {
            return value.clone()
        } else if let Some(enclosing) = &self.enclosing{
            return enclosing.borrow().get(key);
        } else {
            return Expression::None
        }
    }
}

fn use_directive(name : String, argument : Expression, enclosing: Rc<RefCell<Environment>>) -> Expression{

    match name.as_str() {
        "print" => {
            println!("{}", argument.get_display());
        },

        "print_e" => {
            println!("{}", evaluate(argument.clone(), enclosing.clone()).get_display())
        }

        _ => {
            
            

        }
    }

    argument
}

fn beta_reduce(argument_name : String, argument : Expression, body : Expression, enclosing: Rc<RefCell<Environment>>) -> Expression{


    match body { 
        Expression::Unit(ref unit_name) => {
            if *unit_name == argument_name{
                argument
            } else {
                body
            }
        }
        Expression::Grp { inner } => {
            beta_reduce(argument_name, argument, *inner, enclosing.clone())
        }
        Expression::Def { argument: other_argument, body} => {
            

            Expression::Def {
                argument : other_argument.clone(),
                body : Box::new(beta_reduce(argument_name, argument, *body, enclosing.clone()))
            }
        }
        Expression::App { left, right } => {
            let new_left = beta_reduce(argument_name.clone(), argument.clone(), *left, enclosing.clone());
            let new_right = beta_reduce(argument_name, argument, *right, enclosing.clone());

            evaluate(Expression::App{
                left : Box::new(new_left),
                right : Box::new(new_right)
            }, enclosing.clone())
        },

        _ => body
    }

}

fn apply(left : Expression, right : Expression, enclosing: Rc<RefCell<Environment>>) -> Expression{

    let left = evaluate(left, enclosing.clone());
    let right = evaluate(right, enclosing.clone());


    match left {
        Expression::App { left, right } =>{
            let eval_left = evaluate(*left, enclosing.clone());
            let eval_right = evaluate(*right, enclosing.clone());


            apply(eval_left, eval_right, enclosing.clone()) 
        }
        Expression::Def { argument, body } => { 
            let s = beta_reduce(argument.clone(), right, *body, enclosing.clone());


            s
        }
        Expression::Directive { name } => {
            use_directive(name, right, enclosing.clone())
        }
        _ => left
    }

}

pub fn evaluate(expr : Expression, enclosing: Rc<RefCell<Environment>>) -> Expression{


    match expr{
        Expression::Global { list } => {
            list.iter().for_each(|x|{
                evaluate(x.clone(), enclosing.clone());
            });

            Expression::None
        }
        Expression::App { left, right } => {
            apply(*left, *right, enclosing.clone())
        }
        Expression::Grp { inner } => {
            evaluate(*inner, enclosing.clone())
        }

        _ => expr
    }  

}
