use crate::tokenizer::Token;
use crate::compiler;

#[derive(Debug)]
pub struct Program<'a> {
    functions: Vec<CodeFunction<'a>>,
    processes: Vec<CodeProcess<'a>>,
}

#[derive(Debug)]
pub struct CodeFunction<'a> {
    name: String,
    parameters: Vec<CodeVar>,
    body: CodeBlock<'a>
}

#[derive(Debug)]
pub struct CodeProcess<'a> {
    name: String,
    parameters: Vec<CodeVar>,
    body: CodeBlock<'a>
}

#[derive(Debug)]
pub struct CodeBlock<'a> {
    statements: Vec<CodeStatement<'a>>
}

#[derive(Debug)]
pub enum CodeStatement<'a> {
    If { condition: CodeExpression<'a, bool>, if_true: CodeBlock<'a>, if_false: Option<CodeBlock<'a>> },
    While { condition: CodeExpression<'a, bool>, block: CodeBlock<'a> },
    VarInit { var: CodeVar, value: Option<CodeExpression<'a, CodeType>> },
}

#[derive(Debug)]
pub struct CodeVar {
    name: String,
    vartype: CodeType
}

#[derive(Debug)]
pub struct CodeType {
    name: String
}

#[derive(Debug)]
enum AnyNum {
    Int(i64),
    Num(f64)
}

#[derive(Debug)]
pub enum CodeExpression<'a, T> { // All sub-expressions will last as long as this one.
    Raw(T),

    Sum(&'a CodeExpression<'a, AnyNum>, &'a CodeExpression<'a, AnyNum>),
    Difference(&'a CodeExpression<'a, AnyNum>, &'a CodeExpression<'a, AnyNum>),
    Negate(&'a CodeExpression<'a, AnyNum>),
    Product(&'a CodeExpression<'a, AnyNum>, &'a CodeExpression<'a, AnyNum>),
    Quotient(&'a CodeExpression<'a, AnyNum>, &'a CodeExpression<'a, AnyNum>),
    Modulo(&'a CodeExpression<'a, AnyNum>, &'a CodeExpression<'a, AnyNum>),
    Concat(&'a CodeExpression<'a, String>, &'a CodeExpression<'a, String>),
    And(&'a CodeExpression<'a, bool>, &'a CodeExpression<'a, bool>),
    Or(&'a CodeExpression<'a, bool>, &'a CodeExpression<'a, bool>),
    Not(&'a CodeExpression<'a, bool>),
    Greater(&'a CodeExpression<'a, bool>, &'a CodeExpression<'a, bool>),
    Less(&'a CodeExpression<'a, bool>, &'a CodeExpression<'a, bool>),
    GreaterEquals(&'a CodeExpression<'a, bool>, &'a CodeExpression<'a, bool>),
    LessEquals(&'a CodeExpression<'a, bool>, &'a CodeExpression<'a, bool>),
    Equals(&'a CodeExpression<'a, bool>, &'a CodeExpression<'a, bool>),
    NotEquals(&'a CodeExpression<'a, bool>, &'a CodeExpression<'a, bool>),
}

macro_rules! expect_token {
    ($iter:ident, $iden:ty) => {
        match $iter.next().ok_or(compiler::ErrorType::InvalidTokenError)? {
            t @ iden => Ok(t),
            _ => Err(compiler::ErrorType::InvalidTokenError),
        }
    }
}

pub fn parse_program(tokens: &Vec<Token>) -> Result<Program, compiler::CompileError> {
    let mut iter = tokens.iter().peekable();
    let program = Program { functions: Vec::new(), processes: Vec::new() };

    while let Some(t) = iter.peek() {
        match t {
            Token::VarDef => todo!(),
            Token::FunctionDef => todo!(),
            Token::ProcessDef => todo!(),
            Token::Import => todo!(),
            Token::Annotation(_) => todo!(),
            _ => return Err(compiler::CompileError{ error_type: compiler::ErrorType::InvalidTokenError, location: 0 })
        }
    };

    Ok(program)
}

fn parse_var_definition<I: Iterator<Item = Token>>(iter: &mut I) -> Result<CodeStatement, compiler::ErrorType> {
    expect_token!(iter, Token::VarDef)?;
    let name = match iter.next().ok_or(compiler::ErrorType::InvalidTokenError)? {
        Token::Identifier(t) => t,
        _ => return Err(compiler::ErrorType::InvalidTokenError),
    };
    expect_token!(iter, Token::Equals)?;
    let value = parse_expression(iter);
    // smth
}

fn parse_expression<I: Iterator<Item = Token>>(iter: &mut I) -> Result<CodeStatement, compiler::ErrorType> {

}

