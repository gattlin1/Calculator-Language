
pub struct Lexer {
    input: Vec<char>,
    cursor: usize
}

impl Lexer {
    pub fn new(input: Vec<char>) -> Self{
        Lexer {
            input,
            cursor: 0
        }
    }

    pub fn get_next(self) -> Result<Token, String> {
        unimplemented!
    }

    fn skip_whitespace(self) {
        while self.input_left(0) && self.input[self.cursor].is_whitespace() {
            self.cursor += 1;
        }
    }

    fn input_left(self, pos_from_cursor: usize) -> bool {
        self.cursor + pos_from_cursor < self.input.len()
    }
}

enum Token {
    // Special Operators.
    End,

    // Operators
    Plus, Minus, Star, Slash,
    Bang, Equal, Less, Greater,
    GreaterEqual, LessEqual,
    EqualEqual, BangEqual,

    // Punctuators
    Comma, SemiColon, Caret,
    LParen, RParen,
    LBrace, RBrace,

    Identifier(String), NumberLiteral(f64),

    // Reserved Keywords
    Let,
}