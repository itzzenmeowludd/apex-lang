use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // keywords
    Let,
    Fn,
    If,
    Else,
    While,
    For,
    Return,
    Break,
    Continue,
    Import,
    From,
    As,
    True,
    False,
    Nil,
    And,
    Or,
    Not,
    Struct,
    Enum,
    Match,
    Trait,
    Impl,

    // identifiers & literals
    Ident(String),
    Number(f64),
    String(String),

    // punctuation
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Comma,
    Dot,
    Colon,
    Semicolon,

    // operators
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Bang,
    Equal,
    EqualEqual,
    BangEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    AndAnd,
    OrOr,
    PlusEqual,
    MinusEqual,
    StarEqual,
    SlashEqual,
    Arrow,

    // layout
    Newline,
    Indent,
    Dedent,
    Eof,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub line: usize,
    pub column: usize,
}

impl Token {
    pub fn new(kind: TokenKind, line: usize, column: usize) -> Self {
        Self { kind, line, column }
    }
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use TokenKind::*;
        match self {
            Let => write!(f, "let"),
            Fn => write!(f, "fn"),
            If => write!(f, "if"),
            Else => write!(f, "else"),
            While => write!(f, "while"),
            For => write!(f, "for"),
            Return => write!(f, "return"),
            Break => write!(f, "break"),
            Continue => write!(f, "continue"),
            Import => write!(f, "import"),
            From => write!(f, "from"),
            As => write!(f, "as"),
            True => write!(f, "true"),
            False => write!(f, "false"),
            Nil => write!(f, "nil"),
            And => write!(f, "and"),
            Or => write!(f, "or"),
            Not => write!(f, "not"),
            Struct => write!(f, "struct"),
            Enum => write!(f, "enum"),
            Match => write!(f, "match"),
            Trait => write!(f, "trait"),
            Impl => write!(f, "impl"),
            Ident(s) => write!(f, "{s}"),
            Number(n) => write!(f, "{n}"),
            String(s) => write!(f, "\"{s}\""),
            LParen => write!(f, "("),
            RParen => write!(f, ")"),
            LBrace => write!(f, "{{"),
            RBrace => write!(f, "}}"),
            LBracket => write!(f, "["),
            RBracket => write!(f, "]"),
            Comma => write!(f, ","),
            Dot => write!(f, "."),
            Colon => write!(f, ":"),
            Semicolon => write!(f, ";"),
            Plus => write!(f, "+"),
            Minus => write!(f, "-"),
            Star => write!(f, "*"),
            Slash => write!(f, "/"),
            Percent => write!(f, "%"),
            Bang => write!(f, "!"),
            Equal => write!(f, "="),
            EqualEqual => write!(f, "=="),
            BangEqual => write!(f, "!="),
            Less => write!(f, "<"),
            LessEqual => write!(f, "<="),
            Greater => write!(f, ">"),
            GreaterEqual => write!(f, ">="),
            AndAnd => write!(f, "&&"),
            OrOr => write!(f, "||"),
            PlusEqual => write!(f, "+="),
            MinusEqual => write!(f, "-="),
            StarEqual => write!(f, "*="),
            SlashEqual => write!(f, "/="),
            Arrow => write!(f, "->"),
            Newline => write!(f, "\\n"),
            Indent => write!(f, "<indent>"),
            Dedent => write!(f, "<dedent>"),
            Eof => write!(f, "<eof>"),
        }
    }
}
