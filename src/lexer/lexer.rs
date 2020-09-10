use std::str;
use token::*;

use crate::token::token;
#[derive(Debug, Clone)]
pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: u8,
}

pub fn new(_input: &str) -> Lexer {
    let mut l = Lexer {
        input: String::from(_input),
        position: 0,
        read_position: 0,
        ch: 0,
    };
    l.read_char();
    l
}

pub fn new_token(token_type: TokenType, ch: u8) -> Token {
    Token {
        typ: token_type,
        literal: String::from_utf8(vec![ch]).unwrap(),
    }
}

impl Lexer {
    fn read_char(&mut self) {
        if self.read_position == self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input.as_bytes()[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(mut self) -> token::Token {
        let tok: Token;

        let c = vec![self.ch];
        let ch = str::from_utf8(&c).unwrap();
        match ch {
            "=" => tok = new_token(TokenType::Assign, self.ch),
            ";" => tok = new_token(TokenType::Semicolon, self.ch),
            "(" => tok = new_token(TokenType::LParen, self.ch),
            ")" => tok = new_token(TokenType::RParen, self.ch),
            "," => tok = new_token(TokenType::Comma, self.ch),
            "+" => tok = new_token(TokenType::Plus, self.ch),
            "}" => tok = new_token(TokenType::LBrace, self.ch),
            "0" => {
                tok = {
                    Token {
                        typ: TokenType::Eof,
                        literal: String::from(""),
                    }
                }
            }
            _ => {
                if is_letter(ch.parse::<u8>().unwrap()) {
                    tok = Token{
                        typ: TokenType::Other,
                        literal: self.read_identifier()
                    };
                    return tok;
                } else {
                    tok = new_token(TokenType::Illegal, self.ch)
                }
            }
        }
        self.read_char();
        tok
    }

    fn read_identifier(&self) -> String {
        let position = self.position;
        while is_letter(self.ch) {
            self.clone().read_char()
        }
        String::from_utf8(vec![self.input.as_bytes()[position]]).unwrap()
    }
}

pub fn is_letter(ch: u8) -> bool {
    let c = vec![ch];
    let ch = str::from_utf8(&c).unwrap();
    return "a" <= ch && ch <= "z" || "A" <= ch && ch <= "Z" || ch == "_"
}