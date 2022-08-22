use std::collections::VecDeque;

#[cfg(test)]
use crate::tokenizer;
use crate::expression_parser;
use crate::compiler;


const EXPR_TESTS: &[&str] = &[ "
    2 + 2
" ];



#[test]
fn test_parser() {
    for t in EXPR_TESTS {
        println!("{}\n{:?}\n", t, expression_parser::parse_expression(&mut VecDeque::from(tokenizer::tokenizer(t).unwrap())).unwrap());
    }
}

#[test]
fn test_codeblock() {
    
    let set_var = compiler::CodeBlock { // Create SetVar Block
        id: String::from("block"),
        block: String::from("set_var"),
        args: vec![compiler::ValueItem::Varible(String::from("variable")), compiler::ValueItem::String(String::from("this is a string"))],
        tags: vec![],
        target: None,
        action: String::from("=")
    };

    let text_value_merging = compiler::TagItem { //Create tag "Text Value Merging"
        option: String::from("Add spaces"),
        tag: String::from("Text Value Merging"),
        action: String::from("SendMessage"),
        block: String::from("player_action")
    };

    let alignment_mode = compiler::TagItem { //Create tag "Alignment Mode"
        option: String::from("Centered"),
        tag: String::from("Alignment Mode"),
        action: String::from("SendMessage"),
        block: String::from("player_action")
    };

    let send_message = compiler::CodeBlock { //Create SendMessage Block
        id: String::from("block"),
        block: String::from("player_action"),
        args: vec![compiler::ValueItem::Varible(String::from("variable"))],
        tags: vec![text_value_merging, alignment_mode],
        target: Some(String::from("AllPlayers")),
        action: String::from("SendMessage")
    };
    let function = compiler::EventLine {  // Create Function with other blocks
        blocks: vec![set_var, send_message],
        event: String::from("Join")
    }.to_json();

    println!(r#"{{"blocks":[{}]}}"#, function) // Output (with the "blocks" key)
    // {"author":"Fire","name":"§b§lFunction §3» §bFireFunc","version":1,"code":"{Output}"}
    // Author data
}