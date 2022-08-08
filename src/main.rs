mod tokenizer;
mod parser;
mod compiler;

fn main() {
    println!("Hello, world!");
    tokenizer::tokenizer("\"hello\" ident ;"); // EOF here at ':', add better EOF messages in the future 
}
