mod lexer;
use lexer::*;

fn main(){
    use std::io::prelude::*;

    let file_name = "./examples/textFile.txt".to_string();
    let mut file = std::fs::File::open(&file_name).expect("file not found");
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents).expect("Could not read the file contents");
    drop(file);

    let mut lexer = Lexer::new(file_contents.chars().collect());

    while let Some(token) = lexer.next() {
        match token {
           Ok(t) => println!("{:?}", t),
           Err(msg) => panic!("ERROR: Unexpected character: '{}'", msg),
        };
    }
}