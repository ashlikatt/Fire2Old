use crate::tokenizer::Token;
use crate::compiler;

#[derive(Debug)]
pub enum Path {
    Absolute(Vec<String>), // ex: ["std", "selections", "Selection", "DEFAULT"]. If the specified var/fun/whatever isn't found, that's an error.
    Relative(String),      // ex: "myVar". If it's not found, it will look "up" one scope, and keep doing this until top-level, if not found it will check
                           //     stdlib, if STILL not found, that's an error.
}

#[derive(Debug)]
pub struct Program {
    functions: Vec<CodeFunction>,
    processes: Vec<CodeProcess>,
}

#[derive(Debug)]
pub struct CodeFunction {
    location: Path,
    parameters: Vec<CodeVar>,
    body: CodeBlock
}

#[derive(Debug)]
pub struct CodeProcess {
    location: Path,
    parameters: Vec<CodeVar>,
    body: CodeBlock
}

#[derive(Debug)]
pub struct CodeBlock {
    statements: Vec<CodeStatement>
}

#[derive(Debug)]
pub enum CodeStatement {
    If { condition: CodeExpression<bool>, if_true: CodeBlock, if_false: Option<CodeBlock> },
    While { condition: CodeExpression<bool>, block: CodeBlock },
    VarInit { var: CodeVar, value: Option<CodeExpression<CodeType>> },
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
pub enum CodeExpression<T> { // All sub-expressions will last as long as this one.
    Raw(T),

    Sum(Box<CodeExpression<AnyNum>>, Box<CodeExpression<AnyNum>>),
    Difference(Box<CodeExpression<AnyNum>>, Box<CodeExpression<AnyNum>>),
    Negate(Box<CodeExpression<AnyNum>>),
    Product(Box<CodeExpression<AnyNum>>, Box<CodeExpression<AnyNum>>),
    Quotient(Box<CodeExpression<AnyNum>>, Box<CodeExpression<AnyNum>>),
    Modulo(Box<CodeExpression<AnyNum>>, Box<CodeExpression<AnyNum>>),
    Concat(Box<CodeExpression<String>>, Box<CodeExpression<String>>),
    And(Box<CodeExpression<bool>>, Box<CodeExpression<bool>>),
    Or(Box<CodeExpression<bool>>, Box<CodeExpression<bool>>),
    Not(Box<CodeExpression<bool>>),
    Greater(Box<CodeExpression<bool>>, Box<CodeExpression<bool>>),
    Less(Box<CodeExpression<bool>>, Box<CodeExpression<bool>>),
    GreaterEquals(Box<CodeExpression<bool>>, Box<CodeExpression<bool>>),
    LessEquals(Box<CodeExpression<bool>>, Box<CodeExpression<bool>>),
    Equals(Box<CodeExpression<bool>>, Box<CodeExpression<bool>>),
    NotEquals(Box<CodeExpression<bool>>, Box<CodeExpression<bool>>),
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

