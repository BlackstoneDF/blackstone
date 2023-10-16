use std::{
    iter::{Enumerate, Peekable},
    ops::Range,
    str::Chars,
};

use chumsky::span::SimpleSpan;
use logos::{Logos};

use crate::lexer::{self, Token};

pub struct StringLexer<'a> {
    source: Peekable<Enumerate<Chars<'a>>>,
}

impl<'a> Iterator for StringLexer<'a> {
    type Item = Result<(StringToken, SimpleSpan), StringLexerError>;

    fn next(&mut self) -> Option<Self::Item> {
        let begin = self.source.next();
        if let Some(begin) = begin {
            let size = begin.0;
            let _span = SimpleSpan::new(size, size + 1);
            Some(match begin.1 {
                '$' => self.parse_template(size),
                it => self.parse_string(it, size),
            })
        } else {
            None
        }
    }
}

impl<'a> StringLexer<'a> {
    pub fn new(src: &'a str) -> Self {
        Self {
            source: src.chars().enumerate().peekable(),
        }
    }

    // This is so broken and janky that I will have to rewrite this
    fn parse_template(
        &mut self,
        beginning: usize,
    ) -> Result<(StringToken, SimpleSpan), StringLexerError> {
        let _a :char = 'a';
        let next = self.source.next();
        match next {
            Some((span, next)) => {
                if next == '{' {
                    let (res, adv) = self.parse_expr(beginning);
                    self.source.nth(adv);
                    res
                } else {
                    let mut out = String::from(next);
                    if next.is_ascii_alphabetic() || next == '_' {
                        let mut end = beginning;
                        for (span, next) in self.source.by_ref() {
                            end = span;
                            if next.is_ascii_alphabetic() || next.is_ascii_digit() || next == '_' {
                                out.push(next);
                            } else {
                                break;
                            }
                        }
                        Ok((StringToken::Single(out), SimpleSpan::new(beginning, end)))
                    } else {
                        Ok((StringToken::Dollar, SimpleSpan::new(beginning, span)))
                    }
                }
            }
            None => Err(StringLexerError::UnexpectedEndOfString),
        }
    }

    fn parse_expr(
        &mut self,
        beginning: usize,
    ) -> (Result<(StringToken, SimpleSpan), StringLexerError>, usize) {
        // TODO: Trying to find a better way to collect while keeping state
        let iter = self.source.clone();
        let src: String = iter.map(|a| a.1).collect();
        let lexer = lexer::Token::lexer(&src).spanned();

        let mut nesting: u8 = 0;
        let mut tokens: Vec<(Token, Range<usize>)> = Vec::new();
        let mut latest_range = 0;
        for (token, range) in lexer {
            latest_range = range.end;
            if let Ok(token) = token {
                match token {
                    Token::OpenBrace => nesting += 1,
                    Token::CloseBrace => {
                        if nesting == 0 {}
                        nesting -= 1
                    }
                    _ => {
                        tokens.push((token, range));
                        break;
                    }
                }
            } else {
                panic!("Lexer doesn't have errors");
            }
        }
        (
            Ok((
                StringToken::Multi(tokens),
                SimpleSpan::new(beginning, beginning + latest_range),
            )),
            latest_range,
        )
    }

    fn parse_string(
        &mut self,
        first: char,
        beginning: usize,
    ) -> Result<(StringToken, SimpleSpan), StringLexerError> {
        let mut string = String::from(first);
        let mut in_escape = false;
        let mut end = beginning;
        loop {
            let char = if let Some(char) = self.source.peek() {
                end = char.0;
                char.1
            } else {
                break Ok((StringToken::Normal(string), SimpleSpan::new(beginning, end)));
            };

            if in_escape {
                match char {
                    '$' => string.push('$'),
                    '\\' => string.push('\\'),
                    '"' => string.push('"'),
                    _ => break Err(StringLexerError::InvalidEscape),
                }
                in_escape = false;
            } else {
                match char {
                    '\\' => in_escape = true,
                    '$' => {
                        break Ok((StringToken::Normal(string), SimpleSpan::new(beginning, end)))
                    }
                    '"' => break Err(StringLexerError::UnexpectedQuote),
                    it => {
                        string.push(it);
                    }
                }
            }
            self.source.next();
        }
    }
}

#[derive(Debug)]
pub enum StringLexerError {
    InvalidEscape,
    UnexpectedQuote,
    UnexpectedEndOfString,
}

#[derive(Debug)]
pub enum StringToken {
    Dollar,
    Single(String),
    Multi(Vec<(Token, Range<usize>)>),

    Normal(String),
}
