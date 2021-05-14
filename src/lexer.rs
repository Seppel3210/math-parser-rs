use std::fmt;
use std::{fmt::Debug, iter::Peekable, str::Chars};

#[derive(Clone)]
pub struct Token {
    pub lexeme: String,
    pub position: (usize, usize),
    pub kind: TokenType,
}

impl Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?} {}:{} {:?}",
            self.kind, self.position.0, self.position.1, self.lexeme
        )
    }
}

impl Token {
    fn new(lexeme: String, position: (usize, usize), kind: TokenType) -> Self {
        Token {
            lexeme,
            position,
            kind,
        }
    }

    fn char(lexeme: char, span: (usize, usize), kind: TokenType) -> Self {
        Self::new(lexeme.to_string(), span, kind)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TokenType {
    LeftParen,
    RightParen,
    Minus,
    Plus,
    Slash,
    Star,
    Caret,
    Ident,
    Number,
    FnLn,
    Eof,
}

pub struct Lexer<'a> {
    start_pos: (usize, usize),
    current_pos: (usize, usize),
    chars: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Lexer {
            start_pos: (0, 0),
            current_pos: (0, 0),
            chars: source.chars().peekable(),
        }
    }

    fn ident(&mut self) -> Token {
        let mut lexeme = String::new();
        while let Some('a'..='z' | 'A'..='Z') = self.peek() {
            lexeme.push(self.consume())
        }
        if lexeme == "ln" {
            Token::new(lexeme, self.start_pos, TokenType::FnLn)
        } else {
            Token::new(lexeme, self.start_pos, TokenType::Ident)
        }
    }

    fn number(&mut self) -> Result<Token, LexError> {
        let mut lexeme = String::new();
        while let Some('0'..='9') = self.peek() {
            lexeme.push(self.consume());
        }
        if let Some('.') = self.peek() {
            lexeme.push(self.consume());
            if let Some('0'..='9') = self.peek() {
                lexeme.push(self.consume());
                while let Some('0'..='9') = self.peek() {
                    lexeme.push(self.consume());
                }
            } else {
                return Err(self.err('.'));
            }
        }

        Ok(Token::new(lexeme, self.start_pos, TokenType::Number))
    }

    fn peek(&mut self) -> Option<&char> {
        self.chars.peek()
    }

    fn advance(&mut self) -> Option<char> {
        self.current_pos.1 += 1;
        self.chars.next()
    }

    fn consume(&mut self) -> char {
        self.advance().unwrap()
    }

    pub fn tokens(&mut self) -> Result<Vec<Token>, LexError> {
        use TokenType::*;
        let mut tokens = vec![];
        while let Some(c) = self.peek().copied() {
            self.start_pos = self.current_pos;
            let token = match c {
                '(' => Token::char(self.consume(), self.start_pos, LeftParen),
                ')' => Token::char(self.consume(), self.start_pos, RightParen),
                '+' => Token::char(self.consume(), self.start_pos, Plus),
                '-' => Token::char(self.consume(), self.start_pos, Minus),
                '*' => Token::char(self.consume(), self.start_pos, Star),
                '/' => Token::char(self.consume(), self.start_pos, Slash),
                '^' => Token::char(self.consume(), self.start_pos, Caret),
                'a'..='z' | 'A'..='Z' | '_' => self.ident(),
                '0'..='9' => self.number()?,
                '\n' => {
                    self.start_pos.0 += 1;
                    self.start_pos.1 = 0;
                    self.consume();
                    continue;
                }
                ' ' | '\t' => {
                    self.consume();
                    continue;
                }
                _ => Err(self.err(c))?,
            };
            tokens.push(token);
        }
        tokens.push(Token::new("".to_owned(), self.current_pos, Eof));
        Ok(tokens)
    }

    fn err(&self, unexpected: char) -> LexError {
        LexError {
            unexpected,
            position: self.current_pos,
        }
    }
}

#[derive(Debug)]
pub struct LexError {
    unexpected: char,
    position: (usize, usize),
}
