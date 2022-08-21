/*
    expression      = orcheck, [ '&', expression ];
    orcheck         = andcheck, [ '||', orcheck ];
    andcheck        = equalcheck, [ '&&', andcheck ];
    equalcheck      = comparison, [ eqop, equalcheck ];
    comparison      = sum, [ compop, comparison ];
    sum             = term, [ addop, sum ];
    term            = accessor, [ mulop, term ];
    precheck        = [ '!' ], accessor;
    accessor        = exprvalue, [ accop ];
    exprvalue       = "(", expression, ")" | NUMBER | STRING | BOOLEAN | initialize | raw_dict | raw_list;

    accop           = '[' + expression + ']' | '.' | param_list;
    compop          = '>' | '<' | '>=' | '<=';
    eqop            = '==' | '!=';
    addop           = '+' | '-';
    mulop           = '*' | '/' | '%';
*/

use std::collections::VecDeque;

use crate::{tokenizer::Token, compiler};

#[derive(Debug)]
pub enum Value {
    MethodCall {
        on: Box<Value>,
        name: AnonResourceLoc,
        parameters: Vec<Value>
    },
    FunctionCall {
        fun: AnonResourceLoc,
        parameters: Vec<Value>
    },
    Constructor {
        data_type: AnonResourceLoc,
        parameters: Vec<Value>
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
    Variable(AnonResourceLoc),
    Num(f64),
    Int(i64),
    Boolean(bool),
    ListLiteral, // WIP
    DictLiteral, // WIP
}

#[derive(Debug)]
pub enum AnonResourceLoc {
    Absolute(Vec<String>),
    Relative(String)
}


pub fn parse_expression(queue: &mut VecDeque<Token>) -> Result<Value, compiler::CompileError> {
    let first = parse_or(queue)?;
    if let Some(Token::Concat) = queue.front() { 
        queue.pop_front();
        let second = parse_expression(queue)?;
        Ok(Value::MethodCall { 
            on: Box::new(first), 
            name: AnonResourceLoc::Absolute(vec![String::from("std"),String::from("operators"),String::from("Concatenate"), String::from("concatenate")]), 
            parameters: vec![second]
        })
    } else {
        Ok(first)
    }
}

fn parse_or(queue: &mut VecDeque<Token>) -> Result<Value, compiler::CompileError> {
    let first = parse_and(queue)?;
    if let Some(Token::BoolOr) = queue.front() { 
        queue.pop_front();
        let second = parse_or(queue)?;
        Ok(Value::MethodCall { 
            on: Box::new(first), 
            name: AnonResourceLoc::Absolute(vec![String::from("std"),String::from("operators"),String::from("Concatenate"), String::from("concatenate")]),
            parameters: vec![second]
        })
    } else {
        Ok(first)
    }
}

fn parse_and(queue: &mut VecDeque<Token>) -> Result<Value, compiler::CompileError> {
    let first = parse_eq(queue)?;
    if let Some(Token::BoolAnd) = queue.front() { 
        queue.pop_front();
        let second = parse_and(queue)?;
        Ok(Value::MethodCall { 
            on: Box::new(first), 
            name: AnonResourceLoc::Absolute(vec![String::from("std"),String::from("operators"),String::from("Concatenate"), String::from("concatenate")]),
            parameters: vec![second]
        })
    } else {
        Ok(first)
    }
}

fn parse_eq(queue: &mut VecDeque<Token>) -> Result<Value, compiler::CompileError> {
    let first = parse_comp(queue)?;
    if let Some(t) = queue.front() { 
        match t {
            Token::Equals => {
                queue.pop_front();
                let second = parse_eq(queue)?;
                Ok(Value::Equal(Box::new(Value::MethodCall { 
                    on: Box::new(first), 
                    name: AnonResourceLoc::Absolute(vec![String::from("std"),String::from("operators"),String::from("Comparable"), String::from("compare")]),
                    parameters: vec![second]
                })))
            } 
            Token::NotEquals => {
                queue.pop_front();
                let second = parse_eq(queue)?;
                Ok(Value::NotEqual(Box::new(Value::MethodCall { 
                    on: Box::new(first), 
                    name: AnonResourceLoc::Absolute(vec![String::from("std"),String::from("operators"),String::from("Comparable"), String::from("compare")]),
                    parameters: vec![second]
                })))
            } 
            _ => Ok(first)
        }
    } else {
        Ok(first)
    }
}

fn parse_comp(queue: &mut VecDeque<Token>) -> Result<Value, compiler::CompileError> {
    let first = parse_sum(queue)?;
    if let Some(t) = queue.front() { 
        match t {
            Token::Greater => {
                queue.pop_front();
                let second = parse_comp(queue)?;
                Ok(Value::Greater(Box::new(Value::MethodCall { 
                    on: Box::new(first), 
                    name: AnonResourceLoc::Absolute(vec![String::from("std"),String::from("operators"),String::from("Comparable"), String::from("compare")]),
                    parameters: vec![second]
                })))
            } 
            Token::Less => {
                queue.pop_front();
                let second = parse_comp(queue)?;
                Ok(Value::Less(Box::new(Value::MethodCall { 
                    on: Box::new(first), 
                    name: AnonResourceLoc::Absolute(vec![String::from("std"),String::from("operators"),String::from("Comparable"), String::from("compare")]), 
                    parameters: vec![second]
                })))
            } 
            Token::GreaterEqual => {
                queue.pop_front();
                let second = parse_comp(queue)?;
                Ok(Value::GreaterEqual(Box::new(Value::MethodCall { 
                    on: Box::new(first), 
                    name: AnonResourceLoc::Absolute(vec![String::from("std"),String::from("operators"),String::from("Comparable"), String::from("compare")]),
                    parameters: vec![second]
                })))
            } 
            Token::LessEqual => {
                queue.pop_front();
                let second = parse_comp(queue)?;
                Ok(Value::LessEqual(Box::new(Value::MethodCall { 
                    on: Box::new(first), 
                    name: AnonResourceLoc::Absolute(vec![String::from("std"),String::from("operators"),String::from("Comparable"), String::from("compare")]),
                    parameters: vec![second] 
                })))
            } 
            _ => Ok(first)
        }
    } else {
        Ok(first)
    }
}

fn parse_sum(queue: &mut VecDeque<Token>) -> Result<Value, compiler::CompileError> {
    let first = parse_term(queue)?;
    if let Some(t) = queue.front() { 
        match t {
            Token::Sum => {
                queue.pop_front();
                let second = parse_sum(queue)?;
                Ok(Value::MethodCall { 
                    on: Box::new(first), 
                    name: AnonResourceLoc::Absolute(vec![String::from("std"),String::from("operators"),String::from("Add"), String::from("add")]),
                    parameters: vec![second] 
                })
            } 
            Token::Minus => {
                queue.pop_front();
                let second = parse_sum(queue)?;
                Ok(Value::MethodCall { 
                    on: Box::new(first), 
                    name: AnonResourceLoc::Absolute(vec![String::from("std"),String::from("operators"),String::from("Subtract"), String::from("subtract")]),
                    parameters: vec![second] 
                })
            } 
            _ => Ok(first)
        }
    } else {
        Ok(first)
    }
}

fn parse_term(queue: &mut VecDeque<Token>) -> Result<Value, compiler::CompileError> {
    let first = parse_pre(queue)?;
    if let Some(t) = queue.front() { 
        match t {
            Token::Multiply => {
                queue.pop_front();
                let second = parse_term(queue)?;
                Ok(Value::MethodCall { 
                    on: Box::new(first), 
                    name: AnonResourceLoc::Absolute(vec![String::from("std"),String::from("operators"),String::from("Multiply"), String::from("multiply")]),
                    parameters: vec![second] 
                })
            } 
            Token::Divide => {
                queue.pop_front();
                let second = parse_term(queue)?;
                Ok(Value::MethodCall { 
                    on: Box::new(first), 
                    name: AnonResourceLoc::Absolute(vec![String::from("std"),String::from("operators"),String::from("Divide"), String::from("divide")]),
                    parameters: vec![second] 
                })
            } 
            Token::Modulo => {
                queue.pop_front();
                let second = parse_term(queue)?;
                Ok(Value::MethodCall { 
                    on: Box::new(first), 
                    name: AnonResourceLoc::Absolute(vec![String::from("std"),String::from("operators"),String::from("Modulo"), String::from("modulo")]),
                    parameters: vec![second] 
                })
            } 
            _ => Ok(first)
        }
    } else {
        Ok(first)
    }
}

fn parse_pre(queue: &mut VecDeque<Token>) -> Result<Value, compiler::CompileError> {
    if let Some(t) = queue.front() { 
        match t {
            Token::BoolNot => {
                let first = parse_acc(queue)?;
                Ok(Value::MethodCall { 
                    on: Box::new(first), 
                    name: AnonResourceLoc::Absolute(vec![String::from("std"),String::from("operators"),String::from("Not"), String::from("not")]),
                    parameters: Vec::new()
                })
            } 
            _ => parse_acc(queue)
        }
    } else {
        parse_acc(queue)
    }
}

fn parse_acc(queue: &mut VecDeque<Token>) -> Result<Value, compiler::CompileError> {
    let first = parse_value(queue)?;
    if let Some(t) = queue.front() { 
        match t {
            Token::OpenBracket => {
                queue.pop_front(); // Remove [
                let inside = parse_expression(queue)?;
                if let Some(Token::CloseBracket) = queue.front() {
                    queue.pop_front(); // Remove ]
                    Ok(Value::MethodCall { 
                        on: Box::new(first), 
                        name: AnonResourceLoc::Absolute(vec![String::from("std"),String::from("operators"),String::from("Accessible"), String::from("getKey")]),
                        parameters: vec![inside]
                    })
                } else {
                    Err(compiler::CompileError { error_type: compiler::ErrorType::InvalidTokenError, location: 0 })
                }
            }
            Token::Dot => {
                queue.pop_front(); // Remove .
                let second = queue.pop_front();
                if let Some(Token::Identifier(t)) = second {
                    if let Some(Token::OpenParen) = queue.front() {
                        Ok(Value::MethodCall { on: Box::new(first), name: AnonResourceLoc::Relative(t), parameters: parse_param_list(queue)? })
                    } else {
                        Ok(Value::GetProperty { on: Box::new(first), name: t })
                    }
                } else {
                    Err(compiler::CompileError { error_type: compiler::ErrorType::InvalidTokenError, location: 0 })
                }
            } 
            _ => Ok(first)
        }
    } else {
        Ok(first)
    }
}

fn parse_value(queue: &mut VecDeque<Token>) -> Result<Value, compiler::CompileError> {
    if let Some(token) = queue.front() {
        match token {
            Token::Number(_) => {
                if let Some(Token::Number(n)) = queue.pop_front() {
                    Ok(Value::Num(n))
                } else {
                    Err(compiler::CompileError { error_type: compiler::ErrorType::InternalError, location: 0 })
                }
            }
            Token::Int(_) => {
                if let Some(Token::Int(n)) = queue.pop_front() {
                    Ok(Value::Int(n))
                } else {
                    Err(compiler::CompileError { error_type: compiler::ErrorType::InternalError, location: 0 })
                }
            }
            Token::String(_) => {
                if let Some(Token::String(n)) = queue.pop_front() {
                    Ok(Value::String(n))
                } else {
                    Err(compiler::CompileError { error_type: compiler::ErrorType::InternalError, location: 0 })
                }
            }
            Token::Identifier(_) => {
                let resource = parse_resource_location(queue)?;
                if let Some(Token::OpenParen) = queue.front() {
                    Ok(Value::FunctionCall { fun: resource, parameters: parse_param_list(queue)? })
                } else {
                    Ok(Value::Variable(resource))
                }
            }
            Token::True => {
                queue.pop_front();
                Ok(Value::Boolean(true))
            }
            Token::False => {
                queue.pop_front();
                Ok(Value::Boolean(false))
            }
            Token::OpenBracket => {
                parse_list_literal(queue)
            }
            Token::OpenBrace => {
                parse_dict_literal(queue)
            }
            _ => Err(compiler::CompileError { error_type: compiler::ErrorType::InvalidTokenError, location: 0 })
        }
    } else {
        Err(compiler::CompileError { error_type: compiler::ErrorType::InvalidTokenError, location: 0 })
    }
}

fn parse_param_list(queue: &mut VecDeque<Token>) -> Result<Vec<Value>, compiler::CompileError> {
    if let Some(Token::OpenParen) = queue.pop_front() {
        let mut out = Vec::new();
        loop {
            let next = queue.front();
            match next {
                Some(Token::CloseParen) => {
                    return Ok(out);
                }
                Some(Token::Separator) => {
                    queue.pop_front();
                    if let Some(Token::CloseParen) = queue.front() {
                        return Ok(out);
                    } else {
                        out.push(parse_expression(queue)?);
                    }
                }
                Some(_) => {
                    out.push(parse_expression(queue)?);
                }
                None => {
                    return Err(compiler::CompileError { error_type: compiler::ErrorType::InvalidTokenError, location: 0 });
                }
            }
        }
    } else {
        Err(compiler::CompileError { error_type: compiler::ErrorType::InvalidTokenError, location: 0 })
    }
}

fn parse_resource_location(queue: &mut VecDeque<Token>) -> Result<AnonResourceLoc, compiler::CompileError> {
    if let Some(Token::Identifier(s)) = queue.pop_front() {
        if let Some(Token::Relation) = queue.front() {
            let mut loc = Vec::new();
            loop {
                if let Some(Token::Relation) = queue.pop_front() {
                    if let Some(Token::Identifier(t)) = queue.pop_front() {
                        loc.push(t);
                    } else {
                        return Err(compiler::CompileError { error_type: compiler::ErrorType::InvalidTokenError, location: 0 });
                    };
                } else {
                    return Ok(AnonResourceLoc::Absolute(loc));
                }
            }
        } else {
            Ok(AnonResourceLoc::Relative(s))
        }
    } else {
        Err(compiler::CompileError { error_type: compiler::ErrorType::InvalidTokenError, location: 0 })
    }
}

fn parse_list_literal(queue: &mut VecDeque<Token>) -> Result<Value, compiler::CompileError> {
    if let Some(Token::OpenBracket) = queue.pop_front() {
        let mut values = Vec::<Value>::new();
        loop {
            let next = queue.front();
            match next {
                Some(Token::CloseBracket) => {
                    queue.pop_front();
                    break;
                }
                Some(Token::Separator) => {
                    queue.pop_front();
                    if let Some(Token::CloseBracket) = queue.front() {
                        break;
                    } else {
                        values.push(parse_expression(queue)?);
                    };
                }
                Some(_) => {
                    values.push(parse_expression(queue)?);
                }
                None => {
                    return Err(compiler::CompileError { error_type: compiler::ErrorType::InvalidTokenError, location: 0 });
                }
            };
        };
        let mut current = Value::FunctionCall { fun: AnonResourceLoc::Absolute(vec![String::from("std"),String::from("collections"),String::from("List"), String::from("new")]), parameters: Vec::new() };
        for i in values.into_iter() {
            current = Value::MethodCall { on: Box::new(current), name: AnonResourceLoc::Absolute(vec![String::from("std"),String::from("collections"),String::from("List"), String::from("push")]), parameters: vec![i] }
        }
        Ok(current)
    } else {
        Err(compiler::CompileError { error_type: compiler::ErrorType::InvalidTokenError, location: 0 })
    }
}

fn parse_dict_literal(queue: &mut VecDeque<Token>) -> Result<Value, compiler::CompileError> {
    if let Some(Token::OpenBrace) = queue.pop_front() {
        let mut pairs = Vec::<(String, Value)>::new();
        loop {
            let next = queue.front();
            match next {
                Some(Token::CloseBrace) => {
                    queue.pop_front();
                    break;
                }
                Some(Token::Separator) => {
                    queue.pop_front();
                    if let Some(Token::CloseBrace) = queue.front() {
                        break;
                    } else {
                        if let Some(Token::Identifier(s)) = queue.pop_front() {
                            if let Some(Token::TypeDef) = queue.pop_front() {
                                let expr = parse_expression(queue)?;
                                pairs.push((s,expr));
                            } else {
                                return Err(compiler::CompileError { error_type: compiler::ErrorType::InvalidTokenError, location: 0 });
                            }
                        } else {
                            return Err(compiler::CompileError { error_type: compiler::ErrorType::InvalidTokenError, location: 0 });
                        };
                    };
                }
                Some(_) => {
                    if let Some(Token::Identifier(s)) = queue.pop_front() {
                        if let Some(Token::TypeDef) = queue.pop_front() {
                            let expr = parse_expression(queue)?;
                            pairs.push((s,expr));
                        } else {
                            return Err(compiler::CompileError { error_type: compiler::ErrorType::InvalidTokenError, location: 0 });
                        }
                    } else {
                        return Err(compiler::CompileError { error_type: compiler::ErrorType::InvalidTokenError, location: 0 });
                    };
                }
                None => {
                    return Err(compiler::CompileError { error_type: compiler::ErrorType::InvalidTokenError, location: 0 });
                }
            };
        };
        let mut current = Value::FunctionCall { fun: AnonResourceLoc::Absolute(vec![String::from("std"),String::from("collections"),String::from("Dictionary"), String::from("new")]), parameters: Vec::new() };
        for i in pairs.into_iter() {
            let s = Value::String(i.0);
            let v = i.1;
            current = Value::MethodCall { on: Box::new(current), name: AnonResourceLoc::Absolute(vec![String::from("std"),String::from("collections"),String::from("Dictionary"), String::from("setKey")]), parameters: vec![s,v] }
        }
        Ok(current)
    } else {
        Err(compiler::CompileError { error_type: compiler::ErrorType::InvalidTokenError, location: 0 })
    }
}
