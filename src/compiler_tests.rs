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
        args: vec![compiler::ValueItem::Varible(String::from("varible")), compiler::ValueItem::String(String::from("this is a string"))],
        action: String::from("=")
    };

    let send_message = compiler::CodeBlock{ //Create SendMessage Block
        id: String::from("block"),
        block: String::from("player_action"),
        args: vec![compiler::ValueItem::Varible(String::from("varible"))],
        action: String::from("SendMessage")
    };

    let function = compiler::FunctionLine{  // Create Function with other blocks
        args: vec![compiler::ValueItem::Number(2f64)],
        blocks: vec![set_var, send_message],
        name: String::from("func")
    }.toJSON();

    println!(r#"{{"blocks":[{}]}}"#, function) // Output (with the "blocks" key)
    // {"author":"Fire","name":"§b§lFunction §3» §bFireFunc","version":1,"code":"{Output}"}
    // Author data
}