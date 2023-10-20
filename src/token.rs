#[derive(Clone, Debug)]
pub enum Literal {
    Int(i64),
    Float(f64),
    Str(String),
    /* Char(char), */
    Null,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TokType {
    lParen, rParen, lBrack, rBrack, lCurl, rCurl,
    Semicolon,

    Identifier, String, Int, Float, Char,

    IntType, StrType, FloatType, CharType,

    If, Else, Fn,
    Add, Sub, Mul, Div,
    Def, Set, For, ForEach, While,
    Greater, Less, Not,
    NotEq, Equal, LessEq, GreaterEq,
    True, False,
    Ret,
    Ng, // negate

    Expr, Block, Arr,


    Newline,
    Eof,
}

#[derive(Clone, Debug)]
pub struct Token {
    pub ltype: TokType,
    pub lexeme: String,
    pub literal: Literal,
    pub line: u64,
    pub had_error: bool,
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

