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
    If { condition: CodeExpression<'a>, if_true: CodeBlock<'a>, if_false: Option<CodeBlock<'a>> },
    While { condition: CodeExpression<'a>, block: CodeBlock<'a> },
    VarInit { var: CodeVar, value: Option<CodeExpression<'a>> },
}

#[derive(Debug)]
pub struct CodeVar {
    name: String,
    vartype: CodeType
}

#[derive(Debug)]
pub struct CodeType {

}

#[derive(Debug)]
pub enum CodeExpression<'a> { // All sub-expressions will last as long as this one.
    Sum(&'a CodeExpression<'a>, &'a CodeExpression<'a>),
    Difference(&'a CodeExpression<'a>, &'a CodeExpression<'a>),
    Product(&'a CodeExpression<'a>, &'a CodeExpression<'a>),
    Quotient(&'a CodeExpression<'a>, &'a CodeExpression<'a>),
}

pub fn parse_tokens(tokens: &Vec<Token>) -> Result<Program, compiler::CompileError> {
    let mut iter = tokens.into_iter().peekable();
    let mut program = Program { functions: Vec::new(), processes: Vec::new() };

    while let Some(t) = iter.next() {
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


