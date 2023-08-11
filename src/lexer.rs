use logos::{Logos, Lexer};


// NOTE: When changing keywords, go to the src/parser/mod.rs and change the expr() switch to match that

#[derive(Logos, Clone, PartialEq, Debug)]
#[logos(skip r"[ \t\n\f]+")]
pub enum Token {
    Error,

    // Soft Keywords
    #[token("if")]
    If,
    #[token("fn")]
    FunctionDef,
    #[token("proc")]
    ProcessDef,
    #[token("macro")]
    MacroDef,
    #[token("event")]
    EventDef,

    // Key symbols
    #[token(r"//[^\n]*")]
    Comment,
    #[token("/*", comment)]
    MultilineComment,

    // Single char tokens
    #[token(".")]
    Dot,
    #[token(":")]
    Colon,
    #[token(",")]
    Comma,

    // Open and close things
    #[token("(")]
    OpenParen,
    #[token(")")]
    CloseParen,
    #[token("{")]
    OpenBrace,
    #[token("}")]
    CloseBrace,
    #[token("[")]
    OpenBracket,
    #[token("]")]
    CloseBracket,

    #[regex("[a-zA-Z_][a-zA-Z0-9_]*", extract)]
    Identifier(String),
    #[regex(r"\d+(\.\d+)?")]
    Number,
    #[regex(r#""([^"\\]*(?:\\.[^"\\]*)*)""#)]
    String,
}

fn extract(lexer: &mut Lexer<Token>) -> String {
    lexer.slice().to_string()
}

//by default the logos error type is (). You may want to replace it with a better one.
fn comment<'source>(lexer: &mut Lexer<'source, Token>) -> Result<(), ()> {
    println!("Comment triggered!");
    #[derive(Logos, Debug)]
    enum CommentHelper {
        #[token(r"\*")]
        Open,
        #[token(r"*\")]
        Close,
        #[regex(".")]
        AnythingElse,
    }
    let comment_start = lexer.remainder();
    let mut comment_lexer = CommentHelper::lexer(comment_start);
    let mut depth = 1; //we're already inside a comment, so we start from 1
    while depth != 0 {
        match comment_lexer.next() {
            Some(Ok(CommentHelper::Open)) => depth += 1,
            Some(Ok(CommentHelper::Close)) => depth -= 1,
            Some(Ok(CommentHelper::AnythingElse)) => {}
            Some(Err(_)) => return Ok(()),
            None => return Err(()), //unclosed comment
        }
    }
    let comment_end = comment_lexer.remainder();
    let comment_length = comment_end as *const str as *const () as usize
        - comment_start as *const str as *const () as usize;
    lexer.bump(comment_length);
    Ok(())
}

