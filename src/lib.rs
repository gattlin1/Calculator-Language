pub struct Lexer {
    input: Vec<char>,
    position: usize
}

impl Lexer {
    pub fn new(input: Vec<char>, position: usize) -> Self{
        Lexer {
            input,
            position
        }
    }
}