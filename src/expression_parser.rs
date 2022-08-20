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

use crate::{tokenizer::Token, compiler};

#[derive(Debug)]
pub enum Value {
    MethodCall {
        on: Box<Value>,
        name: Method,
        parameters: Option<Vec<Value>>
    },
    GetProperty {
        on: Box<Value>,
        name: String
    },
    Greater(Box<Value>),
    Less(Box<Value>),
    GreaterEqual(Box<Value>),
    LessEqual(Box<Value>),
    Equal(Box<Value>),
    NotEqual(Box<Value>),
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

fn parse_expression(queue: &mut VecDeque<Token>) -> Result<Value, compiler::CompileError> {
    let first = parse_value(queue)?;
    if let Some(t) = queue.front() { 
        match t {
            Token::OpenBrace => {
                let second = parse_expression(queue)?;
                if let Some(Token::CloseBrace) = queue.front() {
                    queue.pop_front();
                    Ok(Value::MethodCall { 
                        on: Box::new(first), 
                        name: Method::Derive(vec![String::from("std"),String::from("operators"),String::from("Accessible"),String::from("access")]), 
                        parameters: Some(vec![second]) 
                    })
                } else {
                    Err(compiler::CompileError { error_type: compiler::ErrorType::InvalidTokenError, location: 0 }) // 0 for now
                }
            }
            Token::OpenParen => { // TODO THIS
                todo!()
            }
            Token::Dot => {
                queue.pop_front();
                let second = parse_expression(queue)?;
                if let Value::String(s) = second {
                    Ok(Value::GetProperty { 
                        on: Box::new(first), 
                        name: s,
                    })
                } else {
                    Err(compiler::CompileError { error_type: compiler::ErrorType::InvalidTokenError, location: 0 }) // 0 for now
                }
            }
            Token::Multiply => {
                queue.pop_front();
                let second = parse_expression(queue)?;
                Ok(Value::MethodCall { 
                    on: Box::new(first), 
                    name: Method::Derive(vec![String::from("std"),String::from("operators"),String::from("Multiply"),String::from("multiply")]), 
                    parameters: Some(vec![second]) 
                })
            }
            Token::Divide => {
                queue.pop_front();
                let second = parse_expression(queue)?;
                Ok(Value::MethodCall { 
                    on: Box::new(first), 
                    name: Method::Derive(vec![String::from("std"),String::from("operators"),String::from("Divide"),String::from("divide")]), 
                    parameters: Some(vec![second]) 
                })
            }
            Token::Modulo => {
                queue.pop_front();
                let second = parse_expression(queue)?;
                Ok(Value::MethodCall { 
                    on: Box::new(first), 
                    name: Method::Derive(vec![String::from("std"),String::from("operators"),String::from("Modulo"),String::from("modulo")]), 
                    parameters: Some(vec![second]) 
                })
            }
            Token::Sum => {
                queue.pop_front();
                let second = parse_expression(queue)?;
                Ok(Value::MethodCall { 
                    on: Box::new(first), 
                    name: Method::Derive(vec![String::from("std"),String::from("operators"),String::from("Add"),String::from("add")]), 
                    parameters: Some(vec![second]) 
                })
            }
            Token::Minus => {
                queue.pop_front();
                let second = parse_expression(queue)?;
                Ok(Value::MethodCall { 
                    on: Box::new(first), 
                    name: Method::Derive(vec![String::from("std"),String::from("operators"),String::from("Subtract"),String::from("subtract")]), 
                    parameters: Some(vec![second]) 
                })
            }
            Token::Greater => {
                queue.pop_front();
                let second = parse_expression(queue)?;
                Ok(Value::Greater(Box::new(Value::MethodCall { 
                    on: Box::new(first), 
                    name: Method::Derive(vec![String::from("std"),String::from("operators"),String::from("Comparable"),String::from("compare")]), 
                    parameters: Some(vec![second]) 
                })))
            }
            Token::GreaterEqual => {
                queue.pop_front();
                let second = parse_expression(queue)?;
                Ok(Value::GreaterEqual(Box::new(Value::MethodCall { 
                    on: Box::new(first), 
                    name: Method::Derive(vec![String::from("std"),String::from("operators"),String::from("Comparable"),String::from("compare")]), 
                    parameters: Some(vec![second]) 
                })))
            }
            Token::Less => {
                queue.pop_front();
                let second = parse_expression(queue)?;
                Ok(Value::Less(Box::new(Value::MethodCall { 
                    on: Box::new(first), 
                    name: Method::Derive(vec![String::from("std"),String::from("operators"),String::from("Comparable"),String::from("compare")]), 
                    parameters: Some(vec![second]) 
                })))
            }
            Token::LessEqual => {
                queue.pop_front();
                let second = parse_expression(queue)?;
                Ok(Value::LessEqual(Box::new(Value::MethodCall { 
                    on: Box::new(first), 
                    name: Method::Derive(vec![String::from("std"),String::from("operators"),String::from("Comparable"),String::from("compare")]), 
                    parameters: Some(vec![second]) 
                })))
            }
            Token::Equals => {
                queue.pop_front();
                let second = parse_expression(queue)?;
                Ok(Value::Equal(Box::new(Value::MethodCall { 
                    on: Box::new(first), 
                    name: Method::Derive(vec![String::from("std"),String::from("operators"),String::from("Comparable"),String::from("compare")]), 
                    parameters: Some(vec![second]) 
                })))
            }
            Token::NotEquals => {
                queue.pop_front();
                let second = parse_expression(queue)?;    
                Ok(Value::NotEqual(Box::new(Value::MethodCall { 
                    on: Box::new(first), 
                    name: Method::Derive(vec![String::from("std"),String::from("operators"),String::from("Comparable"),String::from("compare")]), 
                    parameters: Some(vec![second]) 
                })))
            }
            Token::BoolAnd => {
                queue.pop_front();
                let second = parse_expression(queue)?; 
                Ok(Value::MethodCall { 
                    on: Box::new(first), 
                    name: Method::Derive(vec![String::from("std"),String::from("operators"),String::from("And"),String::from("and")]), 
                    parameters: Some(vec![second]) 
                })
            }
            Token::BoolOr => {
                queue.pop_front();
                let second = parse_expression(queue)?;
                Ok(Value::MethodCall { 
                    on: Box::new(first), 
                    name: Method::Derive(vec![String::from("std"),String::from("operators"),String::from("Or"),String::from("or")]), 
                    parameters: Some(vec![second]) 
                })
            }
            _ => Ok(first) // Nothing left :3
        }
    } else {
        Ok(first)
    }
}

fn parse_value(queue: &mut VecDeque<Token>) -> Result<Value, compiler::CompileError> {
    let token = queue.pop_front();
    // Parse the token into a raw Value
    todo!()
}

fn parse_param_list(queue: &mut VecDeque<Token>) -> Result<Value, compiler::CompileError> {
    todo!()
}