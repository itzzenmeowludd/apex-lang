use crate::{ApexError, Result, Token, TokenKind};

pub struct Lexer {
    source: String,
    file: Option<String>,
}

impl Lexer {
    pub fn new(source: impl Into<String>) -> Self {
        Self { source: source.into(), file: None }
    }

    pub fn with_file(mut self, file: impl Into<String>) -> Self {
        self.file = Some(file.into());
        self
    }

    pub fn tokenize(&self) -> Result<Vec<Token>> {
        let mut tokens = Vec::new();
        let mut indent_stack = vec![0usize];

        for (line_idx, raw_line) in self.source.split('\n').enumerate() {
            let line_no = line_idx + 1;
            let line = raw_line.trim_end_matches('\r');

            let (code, indent, is_blank) = strip_comments_and_indent(line);
            if is_blank {
                continue;
            }

            let current_indent = *indent_stack.last().unwrap();
            if indent > current_indent {
                indent_stack.push(indent);
                tokens.push(Token::new(TokenKind::Indent, line_no, 1));
            } else if indent < current_indent {
                while indent < *indent_stack.last().unwrap() {
                    indent_stack.pop();
                    tokens.push(Token::new(TokenKind::Dedent, line_no, 1));
                }
                if indent != *indent_stack.last().unwrap() {
                    return Err(ApexError::new("bad indentation", line_no, 1).with_file_opt(self.file.clone()));
                }
            }

            let mut i = 0usize;
            let chars: Vec<char> = code.chars().collect();
            while i < chars.len() {
                let c = chars[i];
                let col = indent + i + 1;
                if c.is_whitespace() {
                    i += 1;
                    continue;
                }

                let token = match c {
                    '(' => { i += 1; TokenKind::LParen }
                    ')' => { i += 1; TokenKind::RParen }
                    '{' => { i += 1; TokenKind::LBrace }
                    '}' => { i += 1; TokenKind::RBrace }
                    '[' => { i += 1; TokenKind::LBracket }
                    ']' => { i += 1; TokenKind::RBracket }
                    ',' => { i += 1; TokenKind::Comma }
                    '.' => { i += 1; TokenKind::Dot }
                    ':' => { i += 1; TokenKind::Colon }
                    ';' => { i += 1; TokenKind::Semicolon }
                    '+' => {
                        i += 1;
                        if chars.get(i) == Some(&'=') { i += 1; TokenKind::PlusEqual } else { TokenKind::Plus }
                    }
                    '-' => {
                        i += 1;
                        if chars.get(i) == Some(&'=') { i += 1; TokenKind::MinusEqual }
                        else if chars.get(i) == Some(&'>') { i += 1; TokenKind::Arrow }
                        else { TokenKind::Minus }
                    }
                    '*' => {
                        i += 1;
                        if chars.get(i) == Some(&'=') { i += 1; TokenKind::StarEqual } else { TokenKind::Star }
                    }
                    '/' => {
                        i += 1;
                        if chars.get(i) == Some(&'=') { i += 1; TokenKind::SlashEqual } else { TokenKind::Slash }
                    }
                    '%' => { i += 1; TokenKind::Percent }
                    '!' => {
                        i += 1;
                        if chars.get(i) == Some(&'=') { i += 1; TokenKind::BangEqual } else { TokenKind::Bang }
                    }
                    '=' => {
                        i += 1;
                        if chars.get(i) == Some(&'=') { i += 1; TokenKind::EqualEqual } else { TokenKind::Equal }
                    }
                    '<' => {
                        i += 1;
                        if chars.get(i) == Some(&'=') { i += 1; TokenKind::LessEqual } else { TokenKind::Less }
                    }
                    '>' => {
                        i += 1;
                        if chars.get(i) == Some(&'=') { i += 1; TokenKind::GreaterEqual } else { TokenKind::Greater }
                    }
                    '&' => {
                        i += 1;
                        if chars.get(i) == Some(&'&') { i += 1; TokenKind::AndAnd } else {
                            return Err(ApexError::new("unexpected '&'", line_no, col).with_file_opt(self.file.clone()));
                        }
                    }
                    '|' => {
                        i += 1;
                        if chars.get(i) == Some(&'|') { i += 1; TokenKind::OrOr } else {
                            return Err(ApexError::new("unexpected '|'", line_no, col).with_file_opt(self.file.clone()));
                        }
                    }
                    '"' => {
                        i += 1;
                        let mut out = String::new();
                        while i < chars.len() {
                            let ch = chars[i];
                            if ch == '"' {
                                i += 1;
                                break;
                            }
                            if ch == '\\' {
                                i += 1;
                                let esc = *chars.get(i).ok_or_else(|| ApexError::new("unterminated string", line_no, col).with_file_opt(self.file.clone()))?;
                                let resolved = match esc {
                                    'n' => '\n',
                                    't' => '\t',
                                    'r' => '\r',
                                    '"' => '"',
                                    '\\' => '\\',
                                    other => other,
                                };
                                out.push(resolved);
                                i += 1;
                            } else {
                                out.push(ch);
                                i += 1;
                            }
                        }
                        TokenKind::String(out)
                    }
                    ch if ch.is_ascii_digit() => {
                        let start = i;
                        i += 1;
                        while i < chars.len() && (chars[i].is_ascii_digit() || chars[i] == '_') { i += 1; }
                        if i < chars.len() && chars[i] == '.' && chars.get(i + 1).map_or(false, |c| c.is_ascii_digit()) {
                            i += 1;
                            while i < chars.len() && (chars[i].is_ascii_digit() || chars[i] == '_') { i += 1; }
                        }
                        let s = chars[start..i].iter().collect::<String>().replace('_', "");
                        let num = s.parse::<f64>().map_err(|_| ApexError::new("bad number", line_no, col).with_file_opt(self.file.clone()))?;
                        TokenKind::Number(num)
                    }
                    ch if is_ident_start(ch) => {
                        let start = i;
                        i += 1;
                        while i < chars.len() && is_ident_continue(chars[i]) { i += 1; }
                        let ident = chars[start..i].iter().collect::<String>();
                        keyword_or_ident(&ident)
                    }
                    _ => {
                        return Err(ApexError::new(format!("unexpected character '{c}'"), line_no, col).with_file_opt(self.file.clone()));
                    }
                };

                tokens.push(Token::new(token, line_no, col));
            }

            tokens.push(Token::new(TokenKind::Newline, line_no, line.len().saturating_add(1)));
        }

        while indent_stack.len() > 1 {
            indent_stack.pop();
            tokens.push(Token::new(TokenKind::Dedent, self.source.lines().count().max(1), 1));
        }

        tokens.push(Token::new(TokenKind::Eof, self.source.lines().count().max(1), 1));
        Ok(tokens)
    }
}

fn strip_comments_and_indent(line: &str) -> (String, usize, bool) {
    let mut indent = 0usize;
    let mut i = 0usize;
    let chars: Vec<char> = line.chars().collect();
    while i < chars.len() && (chars[i] == ' ' || chars[i] == '\t') {
        indent += if chars[i] == '\t' { 4 } else { 1 };
        i += 1;
    }

    let mut out = String::new();
    let mut in_str = false;
    let mut escaped = false;
    while i < chars.len() {
        let c = chars[i];
        if in_str {
            out.push(c);
            if escaped {
                escaped = false;
            } else if c == '\\' {
                escaped = true;
            } else if c == '"' {
                in_str = false;
            }
            i += 1;
            continue;
        }

        if c == '"' {
            in_str = true;
            out.push(c);
            i += 1;
            continue;
        }

        if c == '#' {
            break;
        }
        if c == '/' && i + 1 < chars.len() && chars[i + 1] == '/' {
            break;
        }

        out.push(c);
        i += 1;
    }

    let is_blank = out.trim().is_empty();
    (out, indent, is_blank)
}

fn is_ident_start(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_'
}

fn is_ident_continue(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_'
}

fn keyword_or_ident(s: &str) -> TokenKind {
    match s {
        "let" => TokenKind::Let,
        "fn" => TokenKind::Fn,
        "if" => TokenKind::If,
        "else" => TokenKind::Else,
        "while" => TokenKind::While,
        "for" => TokenKind::For,
        "return" => TokenKind::Return,
        "break" => TokenKind::Break,
        "continue" => TokenKind::Continue,
        "import" => TokenKind::Import,
        "from" => TokenKind::From,
        "as" => TokenKind::As,
        "true" => TokenKind::True,
        "false" => TokenKind::False,
        "nil" => TokenKind::Nil,
        "and" => TokenKind::And,
        "or" => TokenKind::Or,
        "not" => TokenKind::Not,
        "struct" => TokenKind::Struct,
        "enum" => TokenKind::Enum,
        "match" => TokenKind::Match,
        "trait" => TokenKind::Trait,
        "impl" => TokenKind::Impl,
        _ => TokenKind::Ident(s.to_string()),
    }
}

trait WithFileOpt<T> {
    fn with_file_opt(self, file: Option<String>) -> Self;
}

impl WithFileOpt<ApexError> for ApexError {
    fn with_file_opt(mut self, file: Option<String>) -> Self {
        self.file = file;
        self
    }
}
