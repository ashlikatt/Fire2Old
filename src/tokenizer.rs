pub enum Token {
    Relation, // Double Colon ::
    TypeDef, // Single Colon :
    Identifier(String), // Start with [a-z][A-Z] but can contain [a-z][A-Z][0-9] and _
    Seperator, // Comma ,
    If,
    FunctionDef, // "fn" Identifier 
    ProcessDef, // "proc" Identifier
    SaveDef, // "SAVE" Identier
    Annotation, // @<Itendifier>
    OpenBrace,
    CloseBrace,
    OpenBracket,
    CloseBracket,
    OpenPren,
    ClosePren,
    Inc, // ++
    Dec, // --
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

pub fn tokenizer(code: &str) {
    let mut iter = code.chars().peekable();
    loop {
        match iter.next() {
            None => break,
            _ => ()
        }
    }
}