#[derive(Debug)]
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

    fn skip_whitespace(&mut self) {
        while self.input_remaining(0) && self.get_current_pos(0).is_whitespace() {
            self.cursor += 1;
        }
    }

    fn input_remaining(&mut self, pos_from_cursor: usize) -> bool {
        self.cursor + pos_from_cursor < self.input.len()
    }

    fn get_current_pos(&mut self, pos_from_cursor: usize) -> char {
        self.input[self.cursor + pos_from_cursor]
    }

    fn valid_identifier(&mut self) -> bool {
        self.input_remaining(0) && (self.get_current_pos(0).is_alphabetic() ||
        self.get_current_pos(0).is_digit(10) || self.get_current_pos(0) == '_')
    }

    fn valid_num_literal(&mut self) -> bool {
        self.input_remaining(0) && (self.get_current_pos(0).is_digit(10) ||
        self.get_current_pos(0) == '.')
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespace();

        if self.cursor >= self.input.len() {
            return None;
        }

        let token = match self.get_current_pos(0) {
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => Token::Star,
            '/' => Token::Slash,
            '!' => {
                if self.input_remaining(1) && self.get_current_pos(1) == '=' {
                    self.cursor += 1;
                    Token::BangEqual
                } else {
                    Token::Bang
                }
            },
            '=' => {
                if self.input_remaining(1) && self.get_current_pos(1) == '=' {
                    self.cursor += 1;
                    Token::EqualEqual
                } else {
                    Token::Equal
                }
            },
            '<' => {
                if self.input_remaining(1) && self.get_current_pos(1) == '=' {
                    self.cursor += 1;
                    Token::LessEqual
                } else {
                    Token::Less
                }
            },
            '>' => {
                if self.input_remaining(1) && self.get_current_pos(1) == '=' {
                    self.cursor += 1;
                    Token::GreaterEqual
                } else {
                    Token::Greater
                }
            },
            '^' => Token::Caret,
            ',' => Token::Comma,
            ';' => Token::SemiColon,
            '(' => Token::LParen,
            ')' => Token::RParen,
            '{' => Token::LBrace,
            '}' => Token::RBrace,
            'a'..='z' | 'A'..='Z' => {
                let mut id = String::new();
                while self.valid_identifier() {
                    id.push(self.input[self.cursor]);
                    self.cursor += 1;
                }
                self.cursor -= 1;

                if id == "let" {
                    Token::Let
                } else {
                    Token::Identifier(id)
                }
            },
            '0'..='9' => {
                let mut num = String::new();
                let mut decimal_counter = 0;
                num.push(self.get_current_pos(0));
                self.cursor += 1;
                loop {
                    if self.valid_num_literal() {
                        if self.get_current_pos(0) == '.' {
                            decimal_counter += 1;
                            if decimal_counter > 1 {
                                break;
                            }
                        }
                        num.push(self.get_current_pos(0));
                        self.cursor += 1;
                    } else {
                        break;
                    }
                }
                self.cursor -= 1;
                Token::NumberLiteral(num.parse::<f64>().unwrap())
            },
            _  => return None

        };
        self.cursor += 1;

        Some(token)
    }

}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Token {
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