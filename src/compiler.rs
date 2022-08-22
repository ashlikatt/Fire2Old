use std::fmt::format;

use crate::expression_parser::{self, Value};

#[derive(Debug)]
pub struct CompileError {
    pub error_type: ErrorType,
    pub location: usize,
}

pub struct EventLine {
    pub blocks: Vec<CodeBlock>,
    pub event: String
}
impl EventLine {
    pub fn to_json(&self) -> String {
        let mut event_line = format!(r#"{{"id":"block","block":"event","args":{{"items":[]}},"action":"{}"}}"#, self.event); // Setup for FunctionBlock
        for (_, e) in self.blocks.iter().enumerate() {
            event_line.push(','); // Comma for JSON
            event_line.push_str(e.to_json().as_str()); // Push CodeBlock
        };
        event_line
    }
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
    pub target: Option<String>,
    pub action: String
}
impl CodeBlock {
    fn to_json(&self) -> String {
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
        if self.block == "player_action" || self.block == "entity_action" || self.block == "if_player" || self.block == "if_entity" { // Check if block has target possibility.\
            let target = match &self.target {
                None => String::from("Selection"),
                Some(n) => n.to_string()
            };
            format!(r#"{{"id":"{}","block":"{}","args":{},"action":"{}","target":"{}"}}"#, self.id.as_str(), self.block.as_str(), args_str, self.action.as_str(), target ) // Finish for CodeBlock
        } else { format!(r#"{{"id":"{}","block":"{}","args":{},"action":"{}"}}"#, self.id.as_str(), self.block.as_str(), args_str, self.action.as_str()) } // Finish for CodeBlock
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