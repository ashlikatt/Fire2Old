use lazy_static::lazy_static;
use regex::Regex;
use crate::compiler;

#[derive(Debug)]
pub enum Token {
    // Keywords
        // Definitions
            VarDef, // let -> Defines a new variable
            FunctionDef, // fn -> Defines a new function
            ProcessDef, // proc -> Defines a new process
            Import, // import -> Imports from another file
        // Control
            If, // if -> Tests if a bool is true and returns if it is.
            For, // for -> Repeats over a range of values / iterator...?
            While, // while -> Repeats while a condition is true
            Return, // return -> Returns a value from a function
            Break, // break -> Exits the current loop
            Continue, // continue -> Skips the current loop iteration
            Select, // select -> Modifies the selection
    
    // Operators
        // Core
            Relation, // :: -> Manually specifies a selector on a relevant action
            Dot, // . -> Accessing struct values and calling extention functions
            Semicolon, // ; -> Specifies the end of a statement
        // Math
            Increment, // ++ -> Increments a variable
            Decrement, // -- -> Decrements a variable
            Sum, // + -> Adds two nums.
            Minus, // - -> Subtracts two nums. The first is assumed to be 0 if no previous number is present.
            Multiply, // * -> Multiplies two nums.
            Divide, // / -> Divides two nums. If both are ints then the result is rounded down.
            Assign, // = -> Sets a variable to a value
            Concat, // & -> Concatenates two values
        // Boolean
            BoolAnd, // && -> True if both booleans are true
            BoolOr, // || -> False if both booleans are false
            BoolNot, // ! -> Negates a boolean's value
            Equals, // == -> Tests equality between two values
            NotEquals, // != -> Opposite of Equals
            Greater, // > -> Tests if one value is greater than another
            Less, // < -> Tests if one value is less than another
            GreaterEqual, // >= -> Tests if one value is greater than or equal to another
            LessEqual, // >= -> Tests if one value is less than or equal to another
        // Vars
            TypeDef, // : -> Specifies a variable's type, or a function's return type.
            Separator, // , -> Separates arguments in function definitions and calls, and separates 
    
    // Values
        Identifier(String), // Begins with a letter but can contain _s and numbers along with them.
        Annotation(String), // @<identifier> -> Hardcoded flags for the compiler to modify syntax and behavior for certain elements of a program
        String(String), // "thing" -> Constant string
        Number(f64), // 1.25 -> Constant number
        Int(i64), // 1 -> Constant int, will automatically upcast to Num
        True, // true -> Constant true
        False, // false -> Constant false
        SelfType, // self -> References the struct being operated on in an extension function

    // Grouping
        OpenBrace, // { -> Contains code blocks
        CloseBrace, // } -> Contains code blocks
        OpenBracket, // [ -> Indexing, no default purpose except for use with @IndexGet and @IndexSet
        CloseBracket, // ] -> Indexing, no default purpose except for use with @IndexGet and @IndexSet
        OpenParen, // ( -> Function calls, definitions, and used in math
        CloseParen, // ) -> Function calls, definitions, and used in math
}

pub fn tokenizer(code: &str) -> Result<Vec<Token>, compiler::CompileError> {
    lazy_static! {  // This will initialize the vars only once
        static ref IDENTIFIER_REGEX: Regex = Regex::new(r"^([a-zA-Z][a-zA-Z0-9_]*)").unwrap();
        static ref ANNOTATION_REGEX: Regex = Regex::new(r"^@([a-zA-Z][a-zA-Z0-9_]*)").unwrap();
        static ref STRING_REGEX: Regex = Regex::new(r#"^"((?:\\.|[^"\\])*)""#).unwrap();
        static ref SINGLE_STRING_REGEX: Regex = Regex::new(r"^'((?:\\.|[^'\\])*)'").unwrap();
        static ref INT_REGEX: Regex = Regex::new(r"^([0-9_]+)").unwrap();
        static ref NUM_REGEX: Regex = Regex::new(r"^([0-9_]*\.[0-9_]+)").unwrap();
    }

    let mut tokens = Vec::<Token>::new();
    let mut current: usize = 0;
    while current<code.len() { // Loops until no more left. current = current char being looked at.
        let cur_slice = &code[current..];
        if cur_slice.starts_with("=") { tokens.push(Token::Assign); }
        if cur_slice.starts_with("==") { tokens.push(Token::Equals); current += 1; }
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
        else if cur_slice.starts_with("&&") { tokens.push(Token::BoolAnd); current += 1; }
        else if cur_slice.starts_with("||") { tokens.push(Token::BoolOr); current += 1; }
        else if cur_slice.starts_with("!=") { tokens.push(Token::NotEquals); current += 1; }
        else if cur_slice.starts_with("!") { tokens.push(Token::BoolNot);}
        else if cur_slice.starts_with("&") { tokens.push(Token::Concat); }
        else if cur_slice.starts_with(">=") { tokens.push(Token::GreaterEqual); current += 1; }
        else if cur_slice.starts_with("<=") { tokens.push(Token::LessEqual); current += 1; }
        else if cur_slice.starts_with(">") { tokens.push(Token::Greater); }
        else if cur_slice.starts_with("<") { tokens.push(Token::Less); }
        else if cur_slice.starts_with("[") { tokens.push(Token::OpenBracket); }
        else if cur_slice.starts_with("]") { tokens.push(Token::CloseBracket); }
        else if cur_slice.starts_with("(") { tokens.push(Token::OpenParen); }
        else if cur_slice.starts_with(")") { tokens.push(Token::CloseParen); }
        else if cur_slice.starts_with("{") { tokens.push(Token::OpenBrace); }
        else if cur_slice.starts_with("}") { tokens.push(Token::CloseBrace); }
        else if cur_slice.starts_with(",") { tokens.push(Token::Separator); }
        else if cur_slice.starts_with("let") { tokens.push(Token::VarDef); current += 2; }
        else if cur_slice.starts_with("fn") { tokens.push(Token::FunctionDef); current += 1; }
        else if cur_slice.starts_with("proc") { tokens.push(Token::ProcessDef); current += 3; }
        else if cur_slice.starts_with("if") { tokens.push(Token::If); current += 1; }
        else if cur_slice.starts_with("for") { tokens.push(Token::For); current += 2; }
        else if cur_slice.starts_with("while") { tokens.push(Token::While); current += 4; }
        else if cur_slice.starts_with("select") { tokens.push(Token::Select); current += 5; }
        else if cur_slice.starts_with("return") { tokens.push(Token::Return); current += 5; }
        else if cur_slice.starts_with("break") { tokens.push(Token::Break); current += 4; }
        else if cur_slice.starts_with("continue") { tokens.push(Token::Continue); current += 7; }
        else if cur_slice.starts_with("self") { tokens.push(Token::SelfType); current += 3; }
        else if let Some(t) = STRING_REGEX.captures(cur_slice) {
            handle_string(t, &mut current, &mut tokens);
            continue;
        }
        else if let Some(t) = SINGLE_STRING_REGEX.captures(cur_slice) {
            handle_string(t, &mut current, &mut tokens);
            continue;
        }
        else if let Some(t) = INT_REGEX.find(cur_slice) {
            tokens.push(
                Token::Int(str::parse::<i64>(t.as_str()).ok().ok_or(compiler::CompileError{ error_type: compiler::ErrorType::InvalidNumError, location: current })? // If it's valid, push, otherwise return an error.
            ));
            current += t.as_str().len();
            continue;
        }
        else if let Some(t) = NUM_REGEX.find(cur_slice) {
            tokens.push(
                Token::Number(str::parse::<f64>(t.as_str()).ok().ok_or(compiler::CompileError{ error_type: compiler::ErrorType::InvalidNumError, location: current })? // If it's valid, push, otherwise return an error.
            ));
            current += t.as_str().len();
            continue;
        }
        else if cur_slice.starts_with(".") { tokens.push(Token::Dot); }
        else if let Some(t) = IDENTIFIER_REGEX.find(cur_slice) {
            tokens.push(Token::Identifier(t.as_str().to_string()));
            current += t.as_str().len();
            continue;
        }
        else if let Some(t) = ANNOTATION_REGEX.captures(cur_slice) {
            tokens.push(Token::Annotation(t.get(1).unwrap().as_str().to_string()));
            current += t.get(1).unwrap().as_str().len() + 1;
            continue;
        }
        current += 1;
    };

    Ok(tokens)
}

fn handle_string(t: regex::Captures, current: &mut usize, tokens: &mut Vec<Token>) {
    let s = t.get(1).unwrap().as_str().to_string()
        .replace(r"\\", r"\")
        .replace(r"\n", "\n");
    let l = s.len();
    tokens.push(Token::String(s));
    *current += l + 2;
}