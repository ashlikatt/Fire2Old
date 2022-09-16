mod tokenizer;
mod parser;
mod compiler;
mod expression_parser;
mod tests;
mod parser_traits;
mod resources;

use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    println!("In file {}", file_path);
    let r = Path::new(file_path);
    parser::parse_project(r);
}

fn compile_project() {

}

fn compile_file() {
    
}