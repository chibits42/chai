#[derive(Clone, Debug)]
pub enum Literal {
    Int(i64),
    Float(f64),
    Str(String),
    Null,
}

#[derive(Clone, Debug)]
pub enum TokType {
    lParen, rParen, lBrack, rBrack, lCurl, rCurl,
    Semicolon,

    Identifier, String, Number,

    If, Else, Fn,
    Add, Sub, Mul, Div,
    Def, For, ForEach, While,
    Greater, Less, Not,
    NotEq, Equal, LessEq, GreaterEq,
    True, False,

    Eof,
}

#[derive(Clone, Debug)]
pub struct Token {
    ltype: TokType,
    lexeme: String,
    literal: Literal,
    line: u64,
    had_error: bool,
}

impl Token {
    pub fn new(typ: TokType, lexeme: String, literal: Literal, line: u64) -> Self {
        Self {
            ltype: typ,
            lexeme: lexeme,
            literal: literal,
            line: line,
            had_error: false,
        }
    }
}

pub fn error(line: u64, msg: String) {
    report(line, String::from(""), msg);
}

fn report(line: u64, whr: String, message: String) {
    panic!("{}", format!("[line {line}] Error {whr}: {message}"));
}

