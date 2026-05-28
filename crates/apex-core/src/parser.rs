use crate::{
    ApexError, AssignOp, Expr, ImportPath, Literal, Param, Program, Result, Stmt, Token, TokenKind,
};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    file: Option<String>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0, file: None }
    }

    pub fn with_file(mut self, file: impl Into<String>) -> Self {
        self.file = Some(file.into());
        self
    }

    pub fn parse_program(&mut self) -> Result<Program> {
        let mut statements = Vec::new();
        self.skip_separators();
        while !self.is_at_end() {
            statements.push(self.parse_statement()?);
            self.skip_separators();
        }
        Ok(Program { statements })
    }

    fn parse_statement(&mut self) -> Result<Stmt> {
        self.skip_separators();
        match self.peek_kind() {
            Some(TokenKind::Let) => self.parse_let(),
            Some(TokenKind::Fn) => self.parse_fn(),
            Some(TokenKind::If) => self.parse_if(),
            Some(TokenKind::While) => self.parse_while(),
            Some(TokenKind::Return) => self.parse_return(),
            Some(TokenKind::Break) => { self.advance(); Ok(Stmt::Break) }
            Some(TokenKind::Continue) => { self.advance(); Ok(Stmt::Continue) }
            Some(TokenKind::Import) => self.parse_import(),
            Some(TokenKind::LBrace) => {
                self.advance();
                let body = self.parse_block(TokenKind::RBrace)?;
                Ok(Stmt::Block(body))
            }
            Some(TokenKind::Semicolon) | Some(TokenKind::Newline) => {
                self.advance();
                self.parse_statement()
            }
            _ => {
                let expr = self.parse_expression()?;
                self.consume_optional_semicolons();
                Ok(Stmt::Expr(expr))
            }
        }
    }

    fn parse_let(&mut self) -> Result<Stmt> {
        self.consume(TokenKind::Let, "expected 'let'")?;
        let name = self.consume_ident("expected variable name after 'let'")?;
        let ty = if self.matches(TokenKind::Colon) {
            Some(self.parse_type_name()?)
        } else {
            None
        };
        self.consume(TokenKind::Equal, "expected '=' after variable name")?;
        let value = self.parse_expression()?;
        self.consume_optional_semicolons();
        Ok(Stmt::Let { name, ty, value })
    }

    fn parse_fn(&mut self) -> Result<Stmt> {
        self.consume(TokenKind::Fn, "expected 'fn'")?;
        let name = self.consume_ident("expected function name")?;
        self.consume(TokenKind::LParen, "expected '(' after function name")?;
        let mut params = Vec::new();
        if !self.check(TokenKind::RParen) {
            loop {
                let param_name = self.consume_ident("expected parameter name")?;
                let ty = if self.matches(TokenKind::Colon) { Some(self.parse_type_name()?) } else { None };
                params.push(Param { name: param_name, ty });
                if !self.matches(TokenKind::Comma) { break; }
            }
        }
        self.consume(TokenKind::RParen, "expected ')' after parameters")?;
        self.consume_optional_colon_or_newline()?;
        let body = self.parse_block_auto()?;
        Ok(Stmt::Function { name, params, body })
    }

    fn parse_if(&mut self) -> Result<Stmt> {
        self.consume(TokenKind::If, "expected 'if'")?;
        let condition = self.parse_expression()?;
        self.consume_optional_colon_or_newline()?;
        let then_branch = self.parse_block_auto()?;

        let else_branch = if self.matches(TokenKind::Else) {
            if self.check(TokenKind::If) {
                let nested = self.parse_if()?;
                Some(vec![nested])
            } else {
                self.consume_optional_colon_or_newline()?;
                Some(self.parse_block_auto()?)
            }
        } else {
            None
        };

        Ok(Stmt::If { condition, then_branch, else_branch })
    }

    fn parse_while(&mut self) -> Result<Stmt> {
        self.consume(TokenKind::While, "expected 'while'")?;
        let condition = self.parse_expression()?;
        self.consume_optional_colon_or_newline()?;
        let body = self.parse_block_auto()?;
        Ok(Stmt::While { condition, body })
    }

    fn parse_return(&mut self) -> Result<Stmt> {
        self.consume(TokenKind::Return, "expected 'return'")?;
        if self.check(TokenKind::Newline) || self.check(TokenKind::Semicolon) || self.check(TokenKind::Dedent) || self.check(TokenKind::RBrace) || self.check(TokenKind::Eof) {
            self.consume_optional_semicolons();
            return Ok(Stmt::Return(None));
        }
        let value = self.parse_expression()?;
        self.consume_optional_semicolons();
        Ok(Stmt::Return(Some(value)))
    }

    fn parse_import(&mut self) -> Result<Stmt> {
        self.consume(TokenKind::Import, "expected 'import'")?;
        let path = if let Some(TokenKind::String(s)) = self.peek_kind().cloned() {
            self.advance();
            ImportPath::File(s)
        } else {
            let mut parts = Vec::new();
            parts.push(self.consume_ident("expected module name after 'import'")?);
            while self.matches(TokenKind::Dot) {
                parts.push(self.consume_ident("expected identifier after '.'")?);
            }
            ImportPath::Module(parts)
        };
        let alias = if self.matches(TokenKind::As) {
            Some(self.consume_ident("expected alias after 'as'")?)
        } else {
            None
        };
        self.consume_optional_semicolons();
        Ok(Stmt::Import { path, alias })
    }

    fn parse_block_auto(&mut self) -> Result<Vec<Stmt>> {
        if self.matches(TokenKind::LBrace) {
            return self.parse_block(TokenKind::RBrace);
        }
        if self.matches(TokenKind::Newline) {
            self.consume(TokenKind::Indent, "expected indented block after ':' or newline")?;
            let body = self.parse_until_dedent()?;
            return Ok(body);
        }
        if self.matches(TokenKind::Indent) {
            let body = self.parse_until_dedent()?;
            return Ok(body);
        }
        Err(self.error_here("expected block"))
    }

    fn parse_until_dedent(&mut self) -> Result<Vec<Stmt>> {
        let mut body = Vec::new();
        self.skip_separators();
        while !self.check(TokenKind::Dedent) && !self.check(TokenKind::RBrace) && !self.is_at_end() {
            body.push(self.parse_statement()?);
            self.skip_separators();
        }
        if self.check(TokenKind::Dedent) {
            self.advance();
        } else if self.check(TokenKind::RBrace) {
            self.advance();
        }
        Ok(body)
    }

    fn parse_block(&mut self, end: TokenKind) -> Result<Vec<Stmt>> {
        let mut body = Vec::new();
        self.skip_separators();
        while !self.check(end.clone()) && !self.is_at_end() {
            body.push(self.parse_statement()?);
            self.skip_separators();
        }
        self.consume(end, "expected block terminator")?;
        Ok(body)
    }

    fn parse_expression(&mut self) -> Result<Expr> {
        self.parse_assignment()
    }

    fn parse_assignment(&mut self) -> Result<Expr> {
        let expr = self.parse_or()?;
        if self.matches(TokenKind::Equal) {
            let value = self.parse_assignment()?;
            return Ok(Expr::Assign { target: Box::new(expr), op: AssignOp::Assign, value: Box::new(value) });
        }
        if self.matches(TokenKind::PlusEqual) {
            let value = self.parse_assignment()?;
            return Ok(Expr::Assign { target: Box::new(expr), op: AssignOp::AddAssign, value: Box::new(value) });
        }
        if self.matches(TokenKind::MinusEqual) {
            let value = self.parse_assignment()?;
            return Ok(Expr::Assign { target: Box::new(expr), op: AssignOp::SubAssign, value: Box::new(value) });
        }
        if self.matches(TokenKind::StarEqual) {
            let value = self.parse_assignment()?;
            return Ok(Expr::Assign { target: Box::new(expr), op: AssignOp::MulAssign, value: Box::new(value) });
        }
        if self.matches(TokenKind::SlashEqual) {
            let value = self.parse_assignment()?;
            return Ok(Expr::Assign { target: Box::new(expr), op: AssignOp::DivAssign, value: Box::new(value) });
        }
        Ok(expr)
    }

    fn parse_or(&mut self) -> Result<Expr> {
        let mut expr = self.parse_and()?;
        while self.matches_any(&[TokenKind::OrOr, TokenKind::Or]) {
            let op = self.previous_kind().clone();
            let right = self.parse_and()?;
            expr = Expr::Logical { left: Box::new(expr), op, right: Box::new(right) };
        }
        Ok(expr)
    }

    fn parse_and(&mut self) -> Result<Expr> {
        let mut expr = self.parse_equality()?;
        while self.matches_any(&[TokenKind::AndAnd, TokenKind::And]) {
            let op = self.previous_kind().clone();
            let right = self.parse_equality()?;
            expr = Expr::Logical { left: Box::new(expr), op, right: Box::new(right) };
        }
        Ok(expr)
    }

    fn parse_equality(&mut self) -> Result<Expr> {
        let mut expr = self.parse_comparison()?;
        while self.matches_any(&[TokenKind::EqualEqual, TokenKind::BangEqual]) {
            let op = self.previous_kind().clone();
            let right = self.parse_comparison()?;
            expr = Expr::Binary { left: Box::new(expr), op, right: Box::new(right) };
        }
        Ok(expr)
    }

    fn parse_comparison(&mut self) -> Result<Expr> {
        let mut expr = self.parse_term()?;
        while self.matches_any(&[
            TokenKind::Less,
            TokenKind::LessEqual,
            TokenKind::Greater,
            TokenKind::GreaterEqual,
        ]) {
            let op = self.previous_kind().clone();
            let right = self.parse_term()?;
            expr = Expr::Binary { left: Box::new(expr), op, right: Box::new(right) };
        }
        Ok(expr)
    }

    fn parse_term(&mut self) -> Result<Expr> {
        let mut expr = self.parse_factor()?;
        while self.matches_any(&[TokenKind::Plus, TokenKind::Minus]) {
            let op = self.previous_kind().clone();
            let right = self.parse_factor()?;
            expr = Expr::Binary { left: Box::new(expr), op, right: Box::new(right) };
        }
        Ok(expr)
    }

    fn parse_factor(&mut self) -> Result<Expr> {
        let mut expr = self.parse_unary()?;
        while self.matches_any(&[TokenKind::Star, TokenKind::Slash, TokenKind::Percent]) {
            let op = self.previous_kind().clone();
            let right = self.parse_unary()?;
            expr = Expr::Binary { left: Box::new(expr), op, right: Box::new(right) };
        }
        Ok(expr)
    }

    fn parse_unary(&mut self) -> Result<Expr> {
        if self.matches_any(&[TokenKind::Bang, TokenKind::Minus, TokenKind::Not]) {
            let op = self.previous_kind().clone();
            let right = self.parse_unary()?;
            return Ok(Expr::Unary { op, right: Box::new(right) });
        }
        self.parse_call()
    }

    fn parse_call(&mut self) -> Result<Expr> {
        let mut expr = self.parse_primary()?;
        loop {
            if self.matches(TokenKind::LParen) {
                let mut args = Vec::new();
                if !self.check(TokenKind::RParen) {
                    loop {
                        args.push(self.parse_expression()?);
                        if !self.matches(TokenKind::Comma) { break; }
                    }
                }
                self.consume(TokenKind::RParen, "expected ')' after arguments")?;
                expr = Expr::Call { callee: Box::new(expr), args };
            } else if self.matches(TokenKind::LBracket) {
                let index = self.parse_expression()?;
                self.consume(TokenKind::RBracket, "expected ']' after index")?;
                expr = Expr::Index { object: Box::new(expr), index: Box::new(index) };
            } else if self.matches(TokenKind::Dot) {
                let name = self.consume_ident("expected property name after '.'")?;
                expr = Expr::Member { object: Box::new(expr), name };
            } else {
                break;
            }
        }
        Ok(expr)
    }

    fn parse_primary(&mut self) -> Result<Expr> {
        match self.peek_kind().cloned() {
            Some(TokenKind::True) => { self.advance(); Ok(Expr::Literal(Literal::Bool(true))) }
            Some(TokenKind::False) => { self.advance(); Ok(Expr::Literal(Literal::Bool(false))) }
            Some(TokenKind::Nil) => { self.advance(); Ok(Expr::Literal(Literal::Nil)) }
            Some(TokenKind::Number(n)) => { self.advance(); Ok(Expr::Literal(Literal::Number(n))) }
            Some(TokenKind::String(s)) => { self.advance(); Ok(Expr::Literal(Literal::String(s))) }
            Some(TokenKind::Ident(name)) => {
                self.advance();
                if self.check(TokenKind::LBrace) {
                    self.advance();
                    let mut fields = Vec::new();
                    if !self.check(TokenKind::RBrace) {
                        loop {
                            let field = self.consume_ident("expected field name")?;
                            self.consume(TokenKind::Colon, "expected ':' in object literal")?;
                            let value = self.parse_expression()?;
                            fields.push((field, value));
                            if !self.matches(TokenKind::Comma) { break; }
                        }
                    }
                    self.consume(TokenKind::RBrace, "expected '}' after object literal")?;
                    Ok(Expr::Object { name: Some(name), fields })
                } else {
                    Ok(Expr::Variable(name))
                }
            }
            Some(TokenKind::LParen) => {
                self.advance();
                let expr = self.parse_expression()?;
                self.consume(TokenKind::RParen, "expected ')' after expression")?;
                Ok(Expr::Grouping(Box::new(expr)))
            }
            Some(TokenKind::LBracket) => {
                self.advance();
                let mut items = Vec::new();
                if !self.check(TokenKind::RBracket) {
                    loop {
                        items.push(self.parse_expression()?);
                        if !self.matches(TokenKind::Comma) { break; }
                    }
                }
                self.consume(TokenKind::RBracket, "expected ']' after array literal")?;
                Ok(Expr::Array(items))
            }
            _ => Err(self.error_here("expected expression")),
        }
    }

    fn parse_type_name(&mut self) -> Result<String> {
        let mut parts = vec![self.consume_ident("expected type name")?];
        while self.matches(TokenKind::Dot) {
            parts.push(self.consume_ident("expected type name segment")?);
        }
        Ok(parts.join("."))
    }

    fn consume_optional_colon_or_newline(&mut self) -> Result<()> {
        if self.matches(TokenKind::Colon) {
            self.skip_separators();
            return Ok(());
        }
        self.skip_separators();
        Ok(())
    }

    fn consume_optional_semicolons(&mut self) {
        while self.matches_any(&[TokenKind::Semicolon, TokenKind::Newline]) {}
    }

    fn skip_separators(&mut self) {
        while self.matches_any(&[TokenKind::Newline, TokenKind::Semicolon]) {}
    }

    fn matches(&mut self, expected: TokenKind) -> bool {
        if self.check(expected.clone()) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn matches_any(&mut self, kinds: &[TokenKind]) -> bool {
        for kind in kinds {
            if self.check(kind.clone()) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, expected: TokenKind) -> bool {
        self.peek_kind().is_some_and(|k| same_variant(k, &expected))
    }

    fn consume(&mut self, expected: TokenKind, message: &str) -> Result<()> {
        if self.check(expected) {
            self.advance();
            Ok(())
        } else {
            Err(self.error_here(message))
        }
    }

    fn consume_ident(&mut self, message: &str) -> Result<String> {
        match self.peek_kind().cloned() {
            Some(TokenKind::Ident(name)) => {
                self.advance();
                Ok(name)
            }
            _ => Err(self.error_here(message)),
        }
    }

    fn error_here(&self, message: &str) -> ApexError {
        let tok = self.peek();
        let mut err = ApexError::new(message, tok.line, tok.column);
        if let Some(file) = &self.file {
            err.file = Some(file.clone());
        }
        err
    }

    fn parse_number_literal(&mut self) -> Result<Expr> {
        match self.peek_kind().cloned() {
            Some(TokenKind::Number(n)) => {
                self.advance();
                Ok(Expr::Literal(Literal::Number(n)))
            }
            _ => Err(self.error_here("expected number")),
        }
    }

    fn peek(&self) -> &Token {
        self.tokens.get(self.current).unwrap_or_else(|| self.tokens.last().expect("token stream empty"))
    }

    fn peek_kind(&self) -> Option<&TokenKind> {
        Some(&self.peek().kind)
    }

    fn previous_kind(&self) -> &TokenKind {
        &self.tokens[self.current - 1].kind
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        &self.tokens[self.current.saturating_sub(1)]
    }

    fn is_at_end(&self) -> bool {
        matches!(self.peek().kind, TokenKind::Eof)
    }
}

fn same_variant(a: &TokenKind, b: &TokenKind) -> bool {
    use std::mem::discriminant;
    discriminant(a) == discriminant(b)
}
