use std::fmt::format;

use crate::expression_parser::{self, Value};

#[derive(Debug)]
pub struct CompileError {
    pub error_type: ErrorType,
    pub location: usize,
}

pub struct FunctionLine {
    pub args: Vec<ValueItem>,
    pub blocks: Vec<CodeBlock>,
    pub tags: Vec<TagItem>,
    pub name: String
}
impl FunctionLine {
    pub fn to_json(&self) -> String {
        let mut args_str: String = String::from(r#"{"items":["#);  // Setup for ValueItems
        for (pos, e) in self.args.iter().enumerate() {
            if args_str.len() != 10 { args_str.push(',') } // Check if args are not empty and put a comma for JSON
            args_str.push('{');
            args_str.push_str(e.to_json().as_str()); // Push ItemValue in
            args_str.push_str(format!(r#","slot":{}}}"#, pos).as_str()); // Have the slot number for DiamondFire
        };
        for (pos, e) in self.tags.iter().enumerate() { // Setup for TagValues
            if args_str.len() != 10 { args_str.push(',') } // Check if args are not empty and put a comma for JSON
            args_str.push('{');
            args_str.push_str(e.to_json().as_str()); // Push TagValues in
            args_str.push_str(format!(r#","slot":{}}}"#, 27-self.tags.len()+pos).as_str()); // Have the slot number for DiamondFire
        };
        args_str.push_str("]}");
        let mut function_line = format!(r#"{{"id":"block","block":"func","args":{},"data":"{}"}}"#, args_str, self.name.as_str()); // Setup for FunctionBlock
        for (_, e) in self.blocks.iter().enumerate() {
            function_line.push(','); // Comma for JSON
            function_line.push_str(e.to_json().as_str()); // Push CodeBlock
        };
        function_line
    }
}

pub struct CodeBlock {
    pub id: String,
    pub block: String,
    pub args: Vec<ValueItem>,
    pub tags: Vec<TagItem>,
    pub action: String
}
impl CodeBlock {
    pub fn to_json(&self) -> String {
        let mut args_str: String = String::from(r#"{"items":["#); // Setup for ValueItems
        for (pos, e) in self.args.iter().enumerate() {
            if args_str.len() != 10 { args_str.push(',') } // Check if index not zero and put a comma for JSON
            args_str.push('{');
            args_str.push_str(e.to_json().as_str()); // Push ValueItem in
            args_str.push_str(format!(r#","slot":{}}}"#, pos).as_str()); // Have the slot number for DiamondFire
        };
        for (pos, e) in self.tags.iter().enumerate() { // Setup for TagValues
            if args_str.len() != 10 { args_str.push(',') } // Check if args are not empty and put a comma for JSON
            args_str.push('{');
            args_str.push_str(e.to_json().as_str()); // Push TagValues in
            args_str.push_str(format!(r#","slot":{}}}"#, 27-self.tags.len()+pos).as_str()); // Have the slot number for DiamondFire
        };        
        args_str.push_str("]}");
        format!(r#"{{"id":"{}","block":"{}","args":{},"action":"{}"}}"#, self.id.as_str(), self.block.as_str(), args_str, self.action.as_str()) // Setup for CodeBlock
    }
}

pub enum ValueItem {
    Number(f64),
    String(String),
    Varible(String)
}
impl ValueItem {
    fn to_json(&self) -> String {
        match self {
            ValueItem::Number(n) => String::from(format!(r#""item":{{"id":"num","data":{{"name":"{}"}}}}"#,n)),
            ValueItem::String(n) => String::from(format!(r#""item":{{"id":"txt","data":{{"name":"{}"}}}}"#,n)),
            ValueItem::Varible(n) => String::from(format!(r#""item":{{"id":"var","data":{{"name":"{}","scope":"local"}}}}"#,n))
        }
    } 
}

pub struct TagItem {
    pub option: String,
    pub tag: String,
    pub action: String,
    pub block: String
}
impl TagItem {
    fn to_json(&self) -> String {
        String::from(format!(r#""item":{{"id":"bl_tag","data":{{"option":"{}","tag":"{}","action":"{}","block":"{}"}}}}"#, self.option.as_str(), self.tag.as_str(), self.action.as_str(), self.block.as_str()))
    }
}


#[derive(Debug)]
pub enum ErrorType {
    InvalidNumError,
    InvalidTokenError,
    InternalError,
}