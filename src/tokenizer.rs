pub enum Token {
    Relation, // Double Colon ::
    TypeDef, // Single Colon :
    Semicolon,
    Identifier(String), // Start with [a-z][A-Z] but can contain [a-z][A-Z][0-9] and _
    Separator, // Comma ,
    If,
    FunctionDef(String), // "fn" Identifier 
    ProcessDef(String), // "proc" Identifier
    SaveDef, // "SAVE" Identier
    Annotation(String), // @<Itendifier>
    OpenBrace,
    CloseBrace,
    OpenBracket,
    CloseBracket,
    OpenPren,
    ClosePren,
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

impl std::fmt::Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Relation => write!(f, "Relation"),
            Self::TypeDef => write!(f, "TypeDef"),
            Self::Semicolon => write!(f, "Semicolon"),
            Self::Identifier(arg0) => f.debug_tuple("Identifier").field(arg0).finish(),
            Self::Separator => write!(f, "Seperator"),
            Self::If => write!(f, "If"),
            Self::FunctionDef(arg0) => f.debug_tuple("FunctionDef").field(arg0).finish(),
            Self::ProcessDef(arg0) => f.debug_tuple("ProcessDef").field(arg0).finish(),
            Self::SaveDef => write!(f, "SaveDef"),
            Self::Annotation(arg0) => f.debug_tuple("Annotation").field(arg0).finish(),
            Self::OpenBrace => write!(f, "OpenBrace"),
            Self::CloseBrace => write!(f, "CloseBrace"),
            Self::OpenBracket => write!(f, "OpenBracket"),
            Self::CloseBracket => write!(f, "CloseBracket"),
            Self::OpenPren => write!(f, "OpenPren"),
            Self::ClosePren => write!(f, "ClosePren"),
            Self::Increment => write!(f, "Inc"),
            Self::Decrement => write!(f, "Dec"),
            Self::Sum => write!(f, "Sum"),
            Self::Minus => write!(f, "Minus"),
            Self::Multiply => write!(f, "Multiply"),
            Self::Divide => write!(f, "Divide"),
            Self::Assign => write!(f, "Assign"),
            Self::Concat => write!(f, "Concat"),
            Self::String(arg0) => f.debug_tuple("String").field(arg0).finish(),
            Self::Number(arg0) => f.debug_tuple("Number").field(arg0).finish(),
            Self::Int(arg0) => f.debug_tuple("Int").field(arg0).finish(),
            Self::True => write!(f, "True"),
            Self::False => write!(f, "False"),
        }
    }
 // added in for debugging
}

pub fn tokenizer(code: &str) {
    let mut iter = code.chars().peekable();
    let mut tokens = Vec::<Token>::new();
    while let Some(current) = iter.next() { // Loops until no more left. current = current char being looked at.
        match current {
            current if current.is_ascii_alphabetic() => {
                let mut ident = String::new();
                ident.push(current); // Pushes char we skip over

                while let Some(current) = iter.next() {  // Use iter_next_if() probably
                    if current.is_alphanumeric() || current == '_' { // Check [a-z][A-Z][0-9] and _
                        ident.push(current);
                    } else { break; } // Break if not proper
                }

                if ident == String::from("fn") { tokens.push(Token::FunctionDef(ident)) }   // Bad code, replacing with better later
                else if ident == String::from("proc") { tokens.push(Token::ProcessDef(ident))} 
                else if ident == String::from("if") { tokens.push(Token::If)}
                else if ident == String::from("SAVE") { tokens.push(Token::SaveDef)}
                else { tokens.push(Token::Identifier(ident)) } 
            },
            ':' => {
                if let Some(next) = iter.peek() {
                    if next == &':' { tokens.push(Token::Relation); iter.next(); } else { tokens.push(Token::TypeDef); }
                } else { handdle_eof() }
            },
            '+' => {
                if let Some(next) = iter.peek() {
                    if next == &'+' { tokens.push(Token::Increment); iter.next(); } else { tokens.push(Token::Sum); }
                } else { handdle_eof() }
            },
            '-' => {
                if let Some(next) = iter.peek() {
                    if next == &'-' { tokens.push(Token::Decrement); iter.next(); } else { tokens.push(Token::Minus); }
                } else { handdle_eof() }
            },
            '@' => {
                let mut annotation: String = String::new();
                while let Some(current) = iter.next() {  // Use iter_next_if() probably
                    if current.is_alphanumeric() || current == '_' { 
                        annotation.push(current);
                    } else { break; } // Break if not proper
                }
                tokens.push(Token::Annotation(annotation))            
            }
            '\"' => {
                let mut strg: String = String::new();
                while let Some(current) = iter.next() {  // Use iter_next_if() probably
                    if current == '\"' { break }
                    strg.push(current);

                }
                tokens.push(Token::String(strg))              
            }
            '*' => tokens.push(Token::Multiply),
            '/' => tokens.push(Token::Divide),
            '=' => tokens.push(Token::Assign),
            '&' => tokens.push(Token::Concat),
            ',' => tokens.push(Token::Separator),
            '{' => tokens.push(Token::OpenBrace),
            '}' => tokens.push(Token::CloseBrace),
            '[' => tokens.push(Token::OpenBracket),
            ']' => tokens.push(Token::CloseBracket),
            '(' => tokens.push(Token::OpenPren),
            ')' => tokens.push(Token::ClosePren),
            _ => ()
        }
    }
    println!("{:?}", tokens);
}

fn handdle_eof () {
    println!("eof D:")
}