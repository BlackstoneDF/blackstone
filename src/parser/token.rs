use logos::Logos;

#[derive(Logos, Clone, PartialEq)]
enum Token {

    // Keywords
    
    #[token("if")]
    If,
    #[token("func")]
    FunctionDefinition,
    #[token("proc")]
    ProcessDefinition,
    #[token("event")]
    EventDefinition,


    #[regex("[a-zA-Z_][a-zA-Z0-9_]*")] 
    Identifier, //TODO Discuss this with team, note to self: context sensitive keywords
    #[regex(r"\d+(\.\d+)?")] 
    Number,
    #[regex(r#""([^"\\]*(?:\\.[^"\\]*)*)""#)]
    String,

    #[token("(")]
    OpenArgs,
    #[token(")")]
    CloseArgs,
    #[token("{")]
    BlockOpen,
    #[token("}")]
    BlockClose,
    #[token("[")]
    ArrayOpen,
    #[token("]")]
    ArrayClose,

}
