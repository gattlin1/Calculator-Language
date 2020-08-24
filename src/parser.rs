use crate::Lexer;
use crate::Token;

// Rules
//     expr   -> term   [ (‘+’ | ‘-’) term   ]*
//     term   -> factor [ (‘*’ | ‘/’) factor ]*
//     factor -> NumberLiteral | ‘(‘ expr ‘)’
#[derive(Debug)]
pub struct Parser {
    lexer: Lexer,
    token: Option<Token>
}

impl Parser {
    pub fn new(input: Vec<char>) -> Self {
        Parser {
            lexer: Lexer::new(input),
            token: None,
        }
    }

    pub fn parse(&mut self) -> Result<AST, String> {
        self.advance_token();
        self.get_expr()
    }

    fn advance_token(&mut self) {
        self.token = self.lexer.next();
    }

    // expr   -> term   [ (‘+’ | ‘-’) term   ]*
    fn get_expr(&mut self) -> Result<AST, String> {
        let ast = self.get_term()?;

        match self.token {
            Some(Token::Plus) => {
                self.advance_token();
                let term = self.get_term()?;
                Ok(AST::Addition(Box::new(ast), Box::new(term)))
            },
            Some(Token::Minus) => {
                self.advance_token();
                let term = self.get_term()?;
                Ok(AST::Subtract(Box::new(ast), Box::new(term)))
            },
            _ => Err(format!("Expected addition or subraction sign. Found {:?}", self.token))
        }
    }

    // term   -> factor [ (‘*’ | ‘/’) factor ]*
    fn get_term(&mut self) -> Result<AST, String> {
        let ast = self.get_factor()?;

        match self.token {
            Some(Token::Star) => {
                self.advance_token();
                let factor = self.get_factor()?;
                Ok(AST::Multiplication(Box::new(ast), Box::new(factor)))
            },
            Some(Token::Slash) => {
                self.advance_token();
                let factor = self.get_factor()?;
                Ok(AST::Division(Box::new(ast), Box::new(factor)))
            }
            _ => Err(format!("Expected a division or multiplication symbol, Found {:?}", self.token))

        }
    }

    // factor -> NumberLiteral | ‘(‘ expr ‘)’
    fn get_factor(&mut self) -> Result<AST, String> {
        match self.token {
            Some(Token::NumberLiteral(n)) => {
                self.token = self.lexer.next();
                Ok(AST::Number(n))
            }
            Some(Token::LParen) => {
                self.advance_token();
                let ast = self.get_expr()?;
                match self.token {
                    Some(Token::RParen) => {
                        self.advance_token();
                        Ok(ast)
                    },
                    _ => Err(format!("Expected RightParen, Found: {:?}", self.token))
                }
            },
            _ => Err(format!("Expected NumberLiteral or LeftParen, Found: {:?}", self.token))

        }
    }
}

#[derive(Debug)]
pub enum AST {
    Addition(Box<AST>, Box<AST>),
    Subtract(Box<AST>, Box<AST>),
    Division(Box<AST>, Box<AST>),
    Multiplication(Box<AST>, Box<AST>),
    Number(f64)
}