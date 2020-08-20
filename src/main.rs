mod lexer;
use lexer::Lexer;

fn main() {
    let text = "+-/><".to_string().chars().collect();
    let lex = Lexer::new(text);
}
