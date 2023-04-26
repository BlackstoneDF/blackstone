#[derive(Debug, Clone)]
pub struct Token {
    pub at_line: u32,
    pub at_char: u32,
    pub token: TokenType,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum TokenType {
    Identifier(String),
    Text(String),
    Number(String), // this is actually a string because %math, %var, etc. exist'
    Location(i32, i32, i32, i32, i32),
    Block { tokens: Vec<Token> },
    Tuple { tokens: Vec<Token> },
    PercentExpr((String, String)),
    OpenParen,
    CloseParen,
    OpenBraces,
    CloseBraces,
    Equals,
    Dot,
    Semicolon,
    Plus,
    Minus,
    Slash,
    Star,
    NoType,
    Eof,
    Comma,
}

/*
example:
playerEvent.join() {
    player.sendMessage("Hello world!", "This is another text");
}

breaks into (token type wise):
Identifier("playerEvent")
Dot
Identifier("join")
Tuple([])
OpenBraces
Identifier("player")
Dot
Identifier("sendMessage")
Tuple(
    [
        Text("Hello world!""),
        Text("This is another text.")
    ]
)
Semicolon
CloseBraces


 */
