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

impl std::fmt::Debug for Token { // added in for debugging
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Relation => write!(f, "Relation"),
            Self::TypeDef => write!(f, "TypeDef"),
            Self::Identifier(arg0) => f.debug_tuple("Identifier").field(arg0).finish(),
            Self::Seperator => write!(f, "Seperator"),
            Self::If => write!(f, "If"),
            Self::FunctionDef => write!(f, "FunctionDef"),
            Self::ProcessDef => write!(f, "ProcessDef"),
            Self::SaveDef => write!(f, "SaveDef"),
            Self::Annotation => write!(f, "Annotation"),
            Self::OpenBrace => write!(f, "OpenBrace"),
            Self::CloseBrace => write!(f, "CloseBrace"),
            Self::OpenBracket => write!(f, "OpenBracket"),
            Self::CloseBracket => write!(f, "CloseBracket"),
            Self::OpenPren => write!(f, "OpenPren"),
            Self::ClosePren => write!(f, "ClosePren"),
            Self::Inc => write!(f, "Inc"),
            Self::Dec => write!(f, "Dec"),
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
                tokens.push(Token::Identifier(ident));
            },
            _ => ()
        }
    }
    println!("{:?}", tokens);
}

fn handdle_eof () {
    println!("eof D:")
}