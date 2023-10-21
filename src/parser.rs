use crate::token::*;

// pub enum Node {
//     Null,
//
//     // types
//     Num(i64),
//     Float(f64),
//     Str(String),
//     Char(char),
//     Bool(bool),
//
//     Expr(Vec<Node>),
//     Block(Vec<Node>),
//     Arr(Vec<Node>),
//
//     Add(Vec<Node>),
//     Sub(Vec<Node>),
//     Mul(Vec<Node>),
//     Div(Vec<Node>),
//
//     Call(String, Vec<Node>),
//
//     If {
//         cond: Vec<Node>,
//         block: Vec<Node>,
//     },
//
//     Def {
//         name: String,
//         typ: Box<Node>,
//         val: Vec<Node>,
//     },
//
//     Fn {
//         name: String,
//         args: Vec<Node>,
//         ret_type: Box<Node>,
//     },
// }


#[derive(Clone, Debug)]
pub struct Node {
    tok: Token,
    children: Vec<Node>,
}

impl Node {
    pub fn new(tok: Token) -> Self {
        Self {
            tok: tok,
            children: Vec::new(),
        }
    }

    pub fn null() -> Self {
        Self {
            tok: Token::null(),
            children: Vec::new(),
        }
    }
}

pub struct Parser {
    out: Vec<Node>,
    outidx: usize,
    toks: Vec<Token>,
    tokidx: usize,
}

impl Parser {
    pub fn new(toks: Vec<Token>) -> Self {
        Self {
            out: Vec::new(),
            outidx: 0,
            toks: toks,
            tokidx: 0, 
        }
    }

    pub fn parse(&mut self) -> Vec<Node> {
        println!("{:#?}", self.toks);
        self.toks.push(Token::null());
        for i in self.toks.clone() {
            if self.tokidx == self.toks.len() { break; }
            println!("{:?}", i);
            let n = self.parse_expr();
            self.out.push(n);
        }

        self.out.clone()
    }

    pub fn parse_expr(&mut self) -> Node {
        if self.is_at_end() { return Node::null(); }
        //println!("{}", self.tokidx);
        let t = self.toks[self.tokidx].clone();

        match t.literal {
            Literal::Int(_) => {
                return Node::new(Token::new(TokType::Int, t.lexeme, t.literal, t.line));
            },
            Literal::Float(_) => {
                return Node::new(Token::new(TokType::Float, t.lexeme, t.literal, t.line));
            },
            Literal::Str(_) => {
                return Node::new(Token::new(TokType::String, t.lexeme, t.literal, t.line));
            },

            _ => {},
        }

        match t.ltype {
            TokType::Add => return self.parse_add(),


            TokType::lBrack => return self.parse_brack(),

            TokType::Eof => { return Node::new(Token::new(TokType::Eof, String::new(), Literal::Null, 0))},
            TokType::Newline => { return Node::new(Token::new(TokType::Newline, String::new(), Literal::Null, 0)); },
            _ => { return Node::null(); }, //panic!("unrecognized token type '{:?}' while parsing", t.ltype),
        }
    }

    fn parse_add(&mut self) -> Node {
        let mut a = Node::new(Token::new(TokType::Add, String::new(), Literal::Null, self.toks[self.tokidx].line));
        while self.advance().ltype != TokType::Newline {
            a.children.push(self.parse_expr());
        }

        a
    }

    fn parse_brack(&mut self) -> Node {
        let mut o = Node::new(
            Token::new(
                TokType::Expr,
                String::new(),
                Literal::Null,
                self.toks[self.tokidx].line,    
            )
        );
    
        println!("{:#?}", self.peek().ltype);
        while self.peek().ltype != TokType::rBrack && !self.is_at_end() {
            o.children.push(self.parse_expr());
            self.advance();
        }

        self.advance();

        o
    }

    fn advance(&mut self) -> Token {
        let b = self.toks[self.tokidx].clone();
        self.tokidx += 1;
        return b.clone();
    }

    fn peek(&self) -> Token {
        if self.is_at_end() { return Token::null(); }
        return self.toks[self.tokidx].clone();
    }

    fn is_at_end(&self) -> bool {
        return self.tokidx as usize >= self.toks.len() - 1;
    }
} 
