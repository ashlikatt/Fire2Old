mod tokenizer;
mod parser;
mod compiler;

fn main() {
    println!("Hello, world!");
    tokenizer::tokenizer("fn proc SAVE if yay :: : :");
}
