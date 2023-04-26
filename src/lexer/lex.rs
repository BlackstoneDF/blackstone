

use super::tokens::{Token, TokenType};

pub struct Lexer {
    pub input: String,
    pub position: usize,
    pub ch: char,
    pub stream: Vec<Token>,
}

fn is_identifiable(ch: char) -> bool {
    ch.is_ascii_uppercase() || ch.is_ascii_lowercase() || ch == '_' || ch == '.' || ch == '%'
}
fn is_digit(ch: char) -> bool {
    ch.is_ascii_digit() || ch == '.'
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut final_input = String::new();
        for line in input.lines() {
            if !line.trim().starts_with("//") {
                final_input.push_str(line);
                final_input.push('\n');
            }
        }
        let final_input = final_input.trim().to_string();
        Lexer {
            input: final_input,
            position: 0usize,
            ch: '_',
            stream: vec![],
        }
    }

    pub fn read_number(&mut self) -> String {
        let pos = self.position;
        while self.position < self.input.len() && is_digit(self.ch) || self.ch == '.' {
            self.next();
        }
        let chars = self.input.get(pos..self.position).expect("failed to slice");
        chars.into()
    }

    pub fn read_identifier(&mut self) -> String {
        let pos = self.position;
        while self.position < self.input.len() && is_identifiable(self.ch) {
            self.next();
        }
        let chars = self.input.get(pos..self.position).expect("failed to slice");
        chars.into()
    }

    pub fn read_text(&mut self) -> String {
        let pos = self.position;
        self.next();
        while self.position < self.input.len() && self.ch != '"' {
            self.next();
        }
        let chars = self.input.get(pos..self.position).expect("failed to slice");
        let ret: String = chars.into();
        ret.replace('\"', "")
    }

    pub fn read_percent_expression(&mut self) -> (String, String) {
        let pos = self.position;
        self.next();
        while self.position < self.input.len() && self.ch != ')' {
            self.next();
        }
        let chars = self.input.get(pos..self.position).expect("failed to slice");
        let chars = chars.trim_start_matches('%');
        let chars = chars
            .split_once('(')
            .expect("somehow failed to split? catch error later");
        (chars.0.into(), chars.1.into())
    }

    pub fn read_whitespace(&mut self) {
        while self.ch.is_whitespace() {
            self.next();
        }
    }

    pub fn read_token(&mut self) -> TokenType {
        let mut token = TokenType::NoType;
        self.read_whitespace();
        match self.ch {
            '(' => token = TokenType::OpenParen,
            ')' => token = TokenType::CloseParen,
            '"' => token = TokenType::Text(self.read_text()),
            '=' => token = TokenType::Equals,
            ';' => token = TokenType::Semicolon,
            '{' => token = TokenType::OpenBraces,
            '}' => token = TokenType::CloseBraces,
            '\0' => token = TokenType::Eof,
            '/' => token = TokenType::Slash,
            '*' => token = TokenType::Star,
            ',' => token = TokenType::Comma,
            '.' => token = TokenType::Dot,
            '+' => token = TokenType::Plus,
            '-' => token = TokenType::Minus,
            '%' => token = TokenType::PercentExpr(self.read_percent_expression()),
            _ => {
                if self.ch == '%' {
                } else if is_identifiable(self.ch) {
                    return TokenType::Identifier(self.read_identifier());
                } else if is_digit(self.ch) {
                    return TokenType::Number(self.read_number());
                }
            }
        }

        self.next();
        token
    }
}

impl Iterator for Lexer {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        self.position += 1;
        self.ch = self.input.chars().nth(self.position).unwrap_or('\0');
        Some(self.input.chars().nth(self.position).unwrap_or('\0'))
    }
}
