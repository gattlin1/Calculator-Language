mod lexer;
mod parser;
use lexer::*;
use parser::*;

fn main(){
    use std::io::prelude::*;

    // Testing Lexer
    let file_name = "./examples/nospace.txt".to_string();
    let mut file = std::fs::File::open(&file_name).expect("file not found");
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents).expect("Could not read the file contents");
    drop(file);


    let mut lexer = Lexer::new(file_contents.chars().collect());

    while let Some(token) = lexer.next() {
        println!("{:?}", token);
    }

    // Testing Parser
    let file_name = "./examples/parser.txt".to_string();
    let mut file = std::fs::File::open(&file_name).expect("file not found");
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents).expect("Could not read the file contents");
    drop(file);
    let mut tree = Parser::new(file_contents.chars().collect());
    println!("{:?}", tree.parse());
}