use std::collections::HashMap;

use crate::token;

pub struct Scanner {
    src: String,
    tokens: Vec<token::Token>,

    start: u64,
    current: u64,
    line: u64,

    keywords: HashMap<String, token::TokType>,
}

impl Scanner {
    pub fn new(src: String) -> Self {
        Self {
            src: src,
            tokens: Vec::new(),

            start: 0,
            current: 0,
            line: 1,

            keywords: HashMap::from([
                ("if".to_string(), token::TokType::If),
                ("else".to_string(), token::TokType::Else),
                ("fn".to_string(), token::TokType::Fn),
                ("def".to_string(), token::TokType::Def),
                ("set".to_string(), token::TokType::Set),
                ("for".to_string(), token::TokType::For),
                ("foreach".to_string(), token::TokType::ForEach),
                ("while".to_string(), token::TokType::While),
                ("true".to_string(), token::TokType::True),
                ("false".to_string(), token::TokType::False),
                ("int".to_string(), token::TokType::IntType),
                ("float".to_string(), token::TokType::FloatType),
                ("str".to_string(), token::TokType::StrType),
                ("char".to_string(), token::TokType::CharType),
                ("ret".to_string(), token::TokType::Ret),
                ("ng".to_string(), token::TokType::Ng),
            ]),
        }
    }

    fn substring(&mut self, st: String, a: u64, b: u64) -> String {
        st.as_str()[a as usize..b as usize].to_string()
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

        // debug printing:
        // println!("{} {}", c, self.current);

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
 
            '"' => self.string(),

            _ => {
                if c.is_digit(10) {
                    self.number();
                } else if self.is_alpha(c) {
                    self.identifier();
                }
                else { self.char_error(c) }
            },
            //   ^^^ this is bad,,, oh well :3
        }
    }

    fn identifier(&mut self) {
        while self.is_alphanumeric(self.peek()) { self.advance(); }
        
        let text = self.substring(self.src.clone(), self.start, self.current);

        let typ = self.keywords.get(&text);

        match typ {
            Some(i) => self.add_token_type(typ.unwrap().clone()),
            None => self.add_token_type(token::TokType::Identifier),
        } 
    }

    fn number(&mut self) {
        let mut float = false;

        while self.peek().is_digit(10) { self.advance(); }

        if self.peek() == '.' && self.peek_next().is_digit(10) {
            float = true;
            self.advance();
            while self.peek().is_digit(10) { self.advance(); }
        } 

        let buf = self.substring(self.src.clone(), self.start, self.current);

        if float {
            self.add_token(
                token::TokType::Float,
                token::Literal::Float(
                    buf.parse::<f64>().unwrap()
                ),
            );
        } else {
            self.add_token(
                token::TokType::Int,
                token::Literal::Int(
                    buf.parse::<i64>().unwrap()
                ),
            );
        } 
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' { self.line += 1 };
            self.advance();
        }
        
        if self.is_at_end() {
            token::error(self.line, String::from("Unterminated string."));
            return;
        }

        self.advance();

        let val = self.substring(self.src.clone(), self.start + 1, self.current);

        self.add_token(
            token::TokType::String,
            token::Literal::Str(val.to_string()),
        );
    }

    fn is_alpha(&self, c: char) -> bool {
        c.is_alphabetic() || c == '_'
    }

    fn is_alphanumeric(&self, c: char) -> bool {
        self.is_alpha(c) || c.is_digit(10)
    }

    fn peek_next(&mut self) -> char {
        if (self.current + 1) as usize >= self.src.len() { return '\0' }
        return self.src.chars().nth((self.current + 1) as usize).unwrap();
    }

    fn add_token_type(&mut self, typ: token::TokType) {
        self.add_token(typ, token::Literal::Null);
    }

    fn add_token(&mut self, typ: token::TokType, literal: token::Literal) {
        let text = self.substring(self.src.clone(), self.start, self.current); 

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
