#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    Word,               // regular argument
    RedirectOut,        // >  or 1>   (stdout, truncate)
    RedirectAppend,     // >> or 1>>  (stdout, append)
    RedirectErr,        // 2>         (stderr, truncate)
    RedirectErrAppend,  // 2>>        (stderr, append)
    Pipe,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub value: String,
}

impl Token {
    pub fn new(kind: TokenKind, value: impl Into<String>) -> Self {
        Token { kind, value: value.into() }
    }
}