use crate::tokenizer::Token;
use crate::compiler;

#[derive(Debug)]
pub enum CodeSegment {
    Function,
    Process
    // etc
}

pub fn parse_tokens(tokens: &Vec<Token>) -> Result<Vec<CodeSegment>, compiler::CompileError> {
    let mut iter = tokens.into_iter().peekable();
    let mut segments: Vec<CodeSegment> = Vec::new();

    while let Some(t) = iter.next() {


        return Err(compiler::CompileError{ error_type: compiler::ErrorType::InvalidTokenError, location: 0 })
    };

    Ok(segments)
}