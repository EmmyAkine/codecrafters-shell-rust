use crate::token::{Token, TokenKind};

pub struct Tokenizer {
    input: Vec<char>,
    pos: usize,
    tokens: Vec<Token>,
    current: String,
}

impl Tokenizer {
    pub fn tokenize(input: &str) -> Result<Vec<Token>, String> {
        let mut t = Tokenizer {
            input: input.chars().collect(),
            pos: 0,
            tokens: Vec::new(),
            current: String::new(),
        };
        t.run()
    }

    fn run(&mut self) -> Result<Vec<Token>, String> {
        while self.pos < self.input.len() {
            let c = self.input[self.pos];
            match c {
                '\'' => self.read_single_quoted()?,
                '"' => self.read_double_quoted()?,
                '\\' => self.read_unquoted_escape()?,
                ' ' => self.skip_space_and_flush(),
                '|' => self.read_pipe(),
                c if self.is_redirect_start(c) => self.read_redirect(),
                _ => self.consume(),
            }
        }

        // flush any trailing word
        if !self.current.is_empty() {
            self.tokens.push(Token::new(TokenKind::Word, std::mem::take(&mut self.current)));
        }

        Ok(std::mem::take(&mut self.tokens))
    }

    // ── single-quoted: everything literal until closing ' ────────────────────
    fn read_single_quoted(&mut self) -> Result<(), String> {
        self.pos += 1; // skip opening '
        while self.pos < self.input.len() && self.input[self.pos] != '\'' {
            self.current.push(self.input[self.pos]);
            self.pos += 1;
        }

        if self.pos >= self.input.len() {
            return Err("syntax error: unterminated single quote".to_string());
        }
        self.pos += 1; // skip closing '
        Ok(())
    }

    // ── double-quoted: \" and \\ are escapes, everything else literal ─────────
    fn read_double_quoted(&mut self) -> Result<(), String> {
        self.pos += 1; // skip opening "
        while self.pos < self.input.len() && self.input[self.pos] != '"' {
            if self.input[self.pos] == '\\' && self.pos + 1 < self.input.len() {
                let next = self.input[self.pos + 1];
                if next == '"' || next == '\\' {
                    self.pos += 1;
                    self.current.push(self.input[self.pos]);
                    self.pos += 1;
                    continue;
                }
            }
            self.current.push(self.input[self.pos]);
            self.pos += 1;
        }

        if self.pos >= self.input.len() {
            return Err("syntax error: unterminated double quote".to_string());
        }
        self.pos += 1; // skip closing "
        Ok(())
    }

    // ── unquoted backslash: next char is always literal ──────────────────────
    fn read_unquoted_escape(&mut self) -> Result<(), String> {
        self.pos += 1; // skip backslash
        if self.pos >= self.input.len() {
            return Err("syntax error: backslash at end of input".to_string());
        }
        self.current.push(self.input[self.pos]);
        self.pos += 1;
        Ok(())
    }

    // ── space: flush current word (if any) and advance past the space ─────────
    fn skip_space_and_flush(&mut self) {
        if !self.current.is_empty() {
            self.tokens.push(Token::new(TokenKind::Word, std::mem::take(&mut self.current)));
        }
        self.pos += 1;
    }

    // ── redirect operators: > >> 1> 1>> 2> 2>> ───────────────────────────────
    fn is_redirect_start(&self, c: char) -> bool {
        if c == '>' {
            return true;
        }
        (c == '1' || c == '2')
            && self.pos + 1 < self.input.len()
            && self.input[self.pos + 1] == '>'
    }

    fn read_redirect(&mut self) {
        if !self.current.is_empty() {
            self.tokens.push(Token::new(TokenKind::Word, std::mem::take(&mut self.current)));
        }

        let mut fd = "";
        if self.input[self.pos] == '1' || self.input[self.pos] == '2' {
            fd = if self.input[self.pos] == '1' { "1" } else { "2" };
            self.pos += 1; // consume "1" or "2"
        }

        self.pos += 1; // consume '>'

        let append = self.pos < self.input.len() && self.input[self.pos] == '>';
        if append {
            self.pos += 1; // consume second '>'
        }

        let kind = match (fd, append) {
            ("2", true) => TokenKind::RedirectErrAppend,
            ("2", false) => TokenKind::RedirectErr,
            (_, true) => TokenKind::RedirectAppend,
            (_, false) => TokenKind::RedirectOut,
        };

        self.tokens.push(Token::new(kind, ""));
    }

    fn read_pipe(&mut self) {
        if !self.current.is_empty() {
            self.tokens.push(Token::new(TokenKind::Word, std::mem::take(&mut self.current)));
        }
        self.tokens.push(Token::new(TokenKind::Pipe, ""));
        self.pos += 1;
    }

    // ── normal character ─────────────────────────────────────────────────────
    fn consume(&mut self) {
        self.current.push(self.input[self.pos]);
        self.pos += 1;
    }
}