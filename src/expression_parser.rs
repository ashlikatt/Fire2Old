/*
    expression      = andcheck, [ '||', expression ];
    andcheck        = equalcheck, [ '&&', expression ];
    equalcheck      = comparison, [ eqop, expression ];
    comparson       = sum, [ compop, expression ];
    sum             = term, [ addop, expression ];
    term            = accessor, [ mulop, expression ];
    accessor        = [ '!' ] exprvalue, [ accop ];
    exprvalue       = "(", expression, ")" | NUMBER | STRING | BOOLEAN | initialize | raw_dict | raw_list;
    
    accop           = '[' + expression + ']' | '.' | param_list;
    compop          = '>' | '<' | '>=' | '<=';
    eqop            = '==' | '!=';
    mulop           = '*' | '/' | '%';
*/

use std::collections::VecDeque;

use crate::tokenizer::Token;

#[derive(Debug)]
pub enum Value {
    MethodCall {
        on: Box<Value>,
        name: Method,
        parameters: Option<Vec<Value>>
    },
    String(String),
    Num(f32),
    Int(i64),
    Boolean(bool),
    ListLiteral, // WIP
    DictLiteral, // WIP
}

#[derive(Debug)]
pub enum Method {
    Relative(String),       // Runs method of the name from the caller's class
    Derive(Vec<String>),     // Runs method of the name from the caller's class that derives from...
    Absolute(Vec<String>),     // Runs the specific method at that location
}

fn parse_expression(queue: &mut VecDeque<Token>) -> Value {
    let first = parse_and(queue);
    if let Some(Token::BoolOr) = queue.front() { // If there is more to read, and the next symbol is an OR...
        queue.pop_front();                              // Remove the OR
        let second = parse_expression(queue);         // Parse another expression after the OR
        Value::MethodCall { 
            on: Box::new(first), 
            name: Method::Derive(vec![String::from("std"),String::from("operators"),String::from("Or"),String::from("or")]), 
            parameters: Some(vec![second]) 
        }
    } else {
        first
    }
}

fn parse_and(queue: &mut VecDeque<Token>) -> Value {
    let first = parse_equals(queue);
    if let Some(Token::BoolAnd) = queue.front() { 
        queue.pop_front();                            
        let second = parse_expression(queue);    
        Value::MethodCall { 
            on: Box::new(first), 
            name: Method::Derive(vec![String::from("std"),String::from("operators"),String::from("And"),String::from("and")]), 
            parameters: Some(vec![second]) 
        }
    } else {
        first
    }
}

fn parse_equals(queue: &mut VecDeque<Token>) -> Value {
    let first = parse_comparison(queue);
    if let Some(Token::Equals) = queue.front() {
        queue.pop_front();                             
        let second = parse_expression(queue);      
        Value::MethodCall { 
            on: Box::new(first), 
            name: Method::Derive(vec![String::from("std"),String::from("operators"),String::from("Comparable"),String::from("isEqual")]), 
            parameters: Some(vec![second]) 
        }
    } else if let Some(Token::NotEquals) = queue.front() {
        queue.pop_front();                             
        let second = parse_expression(queue);      
        Value::MethodCall { 
            on: Box::new(first), 
            name: Method::Derive(vec![String::from("std"),String::from("operators"),String::from("Comparable"),String::from("isNotEqual")]), 
            parameters: Some(vec![second]) 
        }
    } else {
        first
    }
}

fn parse_comparison(queue: &mut VecDeque<Token>) -> Value {
    let first = parse_sum(queue);
    if let Some(Token::Greater) = queue.front() { 
        queue.pop_front();                            
        let second = parse_expression(queue);     
        Value::MethodCall { 
            on: Box::new(first), 
            name: Method::Derive(vec![String::from("std"),String::from("operators"),String::from("Comparable"),String::from("isGreater")]), 
            parameters: Some(vec![second]) 
        }
    } else if let Some(Token::Less) = queue.front() {
        queue.pop_front();                             
        let second = parse_expression(queue);    
        Value::MethodCall { 
            on: Box::new(first), 
            name: Method::Derive(vec![String::from("std"),String::from("operators"),String::from("Comparable"),String::from("isLess")]), 
            parameters: Some(vec![second]) 
        }
    } else if let Some(Token::GreaterEqual) = queue.front() {
        queue.pop_front();                             
        let second = parse_expression(queue);      
        Value::MethodCall { 
            on: Box::new(first), 
            name: Method::Derive(vec![String::from("std"),String::from("operators"),String::from("Comparable"),String::from("isGreaterEq")]), 
            parameters: Some(vec![second]) 
        }
    } else if let Some(Token::LessEqual) = queue.front() {
        queue.pop_front();                             
        let second = parse_expression(queue);     
        Value::MethodCall { 
            on: Box::new(first), 
            name: Method::Derive(vec![String::from("std"),String::from("operators"),String::from("Comparable"),String::from("isLessEq")]), 
            parameters: Some(vec![second]) 
        }
    } else {
        first
    }
}

fn parse_sum(queue: &mut VecDeque<Token>) -> Value {
    let first = parse_term(queue);
    if let Some(Token::Sum) = queue.front() { 
        queue.pop_front();                             
        let second = parse_expression(queue);     
        Value::MethodCall { 
            on: Box::new(first), 
            name: Method::Derive(vec![String::from("std"),String::from("operators"),String::from("Add"),String::from("add")]), 
            parameters: Some(vec![second]) 
        }
    } else if let Some(Token::Minus) = queue.front() {
        queue.pop_front();                             
        let second = parse_expression(queue);     
        Value::MethodCall { 
            on: Box::new(first), 
            name: Method::Derive(vec![String::from("std"),String::from("operators"),String::from("Subtract"),String::from("subtract")]), 
            parameters: Some(vec![second]) 
        }
    } else {
        first
    }
}

fn parse_term(queue: &mut VecDeque<Token>) -> Value {
    let first = parse_accessor(queue);
    if let Some(Token::Multiply) = queue.front() { 
        queue.pop_front();                             
        let second = parse_expression(queue);      
        Value::MethodCall { 
            on: Box::new(first), 
            name: Method::Derive(vec![String::from("std"),String::from("operators"),String::from("Multiply"),String::from("multiply")]), 
            parameters: Some(vec![second]) 
        }
    } else if let Some(Token::Divide) = queue.front() {
        queue.pop_front();                             
        let second = parse_expression(queue);    
        Value::MethodCall { 
            on: Box::new(first), 
            name: Method::Derive(vec![String::from("std"),String::from("operators"),String::from("Divide"),String::from("divide")]), 
            parameters: Some(vec![second]) 
        }
    } else if let Some(Token::Modulo) = queue.front() {
        queue.pop_front();                             
        let second = parse_expression(queue);     
        Value::MethodCall { 
            on: Box::new(first), 
            name: Method::Derive(vec![String::from("std"),String::from("operators"),String::from("Modulo"),String::from("modulo")]), 
            parameters: Some(vec![second]) 
        }
    } else {
        first
    }
}

fn parse_accessor(queue: &mut VecDeque<Token>) -> Value {
    let first = parse_value(queue);
    // Parse [] here
    // Parse <val>.<ident> here, or <val>.<ident>(<params>)    
    
    unimplemented!()
}


fn parse_value(queue: &mut VecDeque<Token>) -> Value {
    let token = queue.pop_front();
    // Parse the token into a raw Value
    unimplemented!()
}