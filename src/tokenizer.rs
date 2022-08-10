use std::ops::Add;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
pub enum Token {
    Relation, // Double Colon ::
    TypeDef, // Single Colon :
    Semicolon,
    Identifier(String), // Start with [a-z][A-Z] but can contain [a-z][A-Z][0-9] and _
    Separator, // Comma ,
    Select,
    Return,
    Break,
    Continue,
    Wait,
    SelfType,
    If,
    FunctionDef, // "fn" Identifier 
    ProcessDef, // "proc" Identifier
    For, // "fn" Identifier 
    While, // "proc" Identifier
    Annotation(String), // @<Itendifier>
    OpenBrace,
    CloseBrace,
    OpenBracket,
    CloseBracket,
    OpenParen,
    CloseParen,
    Increment, // ++
    Decrement, // --
    Sum, 
    Minus,
    Multiply,
    Divide,
    Assign, // =
    Concat, // &
    String(String),
    Number(f64),
    Int(i64),
    True,
    False
}

#[derive(Debug)]
pub enum CompileError {
    InvalidNum{ loc: usize },
}

pub fn tokenizer(code: &str) -> Result<Vec<Token>, CompileError> {
    lazy_static! {  // This will initialize the vars only once
        static ref IDENTIFIER_REGEX: Regex = Regex::new(r"^([a-zA-Z][a-zA-Z0-9_]*)").unwrap();
        static ref ANNOTATION_REGEX: Regex = Regex::new(r"^@([a-zA-Z][a-zA-Z0-9_]*)").unwrap();
        static ref INT_REGEX: Regex = Regex::new(r"^([0-9_]+)").unwrap();
        static ref NUM_REGEX: Regex = Regex::new(r"^([0-9_]*\.[0-9_]*)").unwrap();
    }

    let mut tokens = Vec::<Token>::new();
    let mut current: usize = 0;
    while current<code.len() { // Loops until no more left. current = current char being looked at.
        let cur_slice = &code[current..];
        if cur_slice.starts_with("=") { tokens.push(Token::Assign); }
        else if cur_slice.starts_with("true") { tokens.push(Token::True); current += 3; }
        else if cur_slice.starts_with("false") { tokens.push(Token::False); current += 4; }
        else if cur_slice.starts_with(";") { tokens.push(Token::Semicolon); }
        else if cur_slice.starts_with("::") { tokens.push(Token::Relation); current += 1; }
        else if cur_slice.starts_with(":") { tokens.push(Token::TypeDef); current += 1; }
        else if cur_slice.starts_with("++") { tokens.push(Token::Increment); current += 1; }
        else if cur_slice.starts_with("--") { tokens.push(Token::Decrement); current += 1; }
        else if cur_slice.starts_with("+") { tokens.push(Token::Sum); }
        else if cur_slice.starts_with("-") { tokens.push(Token::Minus); }
        else if cur_slice.starts_with("*") { tokens.push(Token::Multiply); }
        else if cur_slice.starts_with("/") { tokens.push(Token::Divide); }
        else if cur_slice.starts_with("&") { tokens.push(Token::Concat); }
        else if cur_slice.starts_with("[") { tokens.push(Token::OpenBracket); }
        else if cur_slice.starts_with("]") { tokens.push(Token::CloseBracket); }
        else if cur_slice.starts_with("(") { tokens.push(Token::OpenParen); }
        else if cur_slice.starts_with(")") { tokens.push(Token::CloseParen); }
        else if cur_slice.starts_with("[") { tokens.push(Token::OpenBrace); }
        else if cur_slice.starts_with("]") { tokens.push(Token::CloseBrace); }
        else if cur_slice.starts_with(",") { tokens.push(Token::Separator); }
        else if cur_slice.starts_with("fn") { tokens.push(Token::FunctionDef); current += 1; }
        else if cur_slice.starts_with("proc") { tokens.push(Token::ProcessDef); current += 3; }
        else if cur_slice.starts_with("if") { tokens.push(Token::If); current += 1; }
        else if cur_slice.starts_with("for") { tokens.push(Token::For); current += 2; }
        else if cur_slice.starts_with("while") { tokens.push(Token::While); current += 4; }
        else if cur_slice.starts_with("select") { tokens.push(Token::Select); current += 5; }
        else if cur_slice.starts_with("return") { tokens.push(Token::Return); current += 5; }
        else if cur_slice.starts_with("break") { tokens.push(Token::Break); current += 4; }
        else if cur_slice.starts_with("continue") { tokens.push(Token::Continue); current += 7; }
        else if cur_slice.starts_with("wait") { tokens.push(Token::Wait); current += 3; }
        else if cur_slice.starts_with("self") { tokens.push(Token::SelfType); current += 3; }
        else if let Some(t) = INT_REGEX.find(cur_slice) {
            tokens.push(
                Token::Int(str::parse::<i64>(t.as_str()).ok().ok_or(CompileError::InvalidNum{loc: current})? // If it's valid, push, otherwise return an error.
            ));
            current += t.as_str().len();
            continue;
        }
        else if let Some(t) = NUM_REGEX.find(cur_slice) {
            tokens.push(
                Token::Number(str::parse::<f64>(t.as_str()).ok().ok_or(CompileError::InvalidNum{loc: current})? // If it's valid, push, otherwise return an error.
            ));
            current += t.as_str().len();
            continue;
        }
        else if let Some(t) = IDENTIFIER_REGEX.find(cur_slice) {
            tokens.push(Token::Identifier(t.as_str().to_string()));
            current += t.as_str().len();
            continue;
        }
        else if let Some(t) = ANNOTATION_REGEX.find(cur_slice) {
            tokens.push(Token::Annotation(t.as_str().to_string()));
            current += t.as_str().len();
            continue;
        }
        current += 1;
    };

    Ok(tokens)
}