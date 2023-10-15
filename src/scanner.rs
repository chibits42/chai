use crate::token;

pub struct Scanner {
    src: String,
    tokens: Vec<token::Token>,

    start: u64,
    current: u64,
    line: u64,
}

impl Scanner {
    pub fn new(src: String) -> Self {
        Self {
            src: src,
            tokens: Vec::new(),

            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<token::Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(
            token::Token::new(
                token::TokType::Eof,
                String::new(),
                token::Literal::Null,
                self.line,
            )
        );

        return self.tokens.clone();


    }

    fn scan_token(&mut self) {
        let c: char = self.advance();
        println!("{} {}", c, self.current);
        match c {
            '(' => self.add_token_type(token::TokType::lParen),
            ')' => self.add_token_type(token::TokType::rParen),
            '{' => self.add_token_type(token::TokType::lCurl),
            '}' => self.add_token_type(token::TokType::rCurl),
            '[' => self.add_token_type(token::TokType::lBrack),
            ']' => self.add_token_type(token::TokType::rBrack),
            ';' => self.add_token_type(token::TokType::Semicolon),

            '+' => self.add_token_type(token::TokType::Add),
            '-' => self.add_token_type(token::TokType::Sub),
            '*' => self.add_token_type(token::TokType::Mul),
            '/' => {
                if self.match_next('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token_type(token::TokType::Div);
                }
            },

            '!' => {
                let buf = if self.match_next('=') { token::TokType::NotEq } else { token::TokType::Not }; 
                self.add_token_type(buf);
            },

            '=' => {
                if self.match_next('=') {
                    self.add_token_type(token::TokType::Equal);
                } else { self.char_error(c) }
            },

            '<' => {
                let buf = if self.match_next('=') { token::TokType::LessEq }
                    else { token::TokType::Less };
                self.add_token_type(buf);
            },

            '>' => {
                let buf = if self.match_next('=') { token::TokType::GreaterEq } else { token::TokType::Greater };
                self.add_token_type(buf);
            },

            ' ' | '\r' | '\t' => { },

            '\n' => { self.line += 1 },

            // OO WEE STRINGS N NUMBERS N SHIT


            _ => self.char_error(c),
            //   ^^^ this is bad,,, oh well :3
        }
    }

    fn add_token_type(&mut self, typ: token::TokType) {
        self.add_token(typ, token::Literal::Null);
    }

    fn add_token(&mut self, typ: token::TokType, literal: token::Literal) {
        let text = &self.src.as_str()[self.start as usize..self.current as usize];
        self.tokens.push(token::Token::new(typ, text.to_string(), literal, self.line));
    }

    fn advance(&mut self) -> char { 
        let b = self.src.chars().nth(self.current as usize).unwrap();
        self.current += 1;
        return b;
    }

    fn is_at_end(&self) -> bool {
        return self.current as usize >= self.src.len() - 1;
    }

    fn match_next(&mut self, expected: char) -> bool {
        if self.is_at_end() { return false; }
        if self.src.chars().nth(self.current as usize).unwrap() != expected { return false; }

        self.current += 1;
        return true;
    }

    fn char_error(&self, c: char) {
        token::error(self.line, format!("Unexpected character {c}."));
    }

    fn peek(&self) -> char {
        if self.is_at_end() { return '\0' }
        self.src.chars().nth(self.current as usize).unwrap()
    }
}
