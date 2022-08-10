mod tokenizer;
mod parser;
mod compiler;
mod tests;

fn main() {
    println!("Hello, world!");
    tokenizer::tokenizer("\"hello\" fn;;"); // EOF here at ':', add better EOF messages in the future 
}
