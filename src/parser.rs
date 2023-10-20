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
        for i in self.toks.clone() {
            println!("{:?}", i);
            let n = self.parse_expr();
            self.out.push(n);
        }

        self.out.clone()
    }

    pub fn parse_expr(&mut self) -> Node {
        let t = (&self.toks[self.tokidx]).clone();

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
            TokType::Add => {
                let mut o = Node::new(Token::new(TokType::Add, t.lexeme, t.literal, t.line));
                o.children.push(self.parse_add());
                return o;
            },

            TokType::Eof => { return Node::new(Token::new(TokType::Eof, String::new(), Literal::Null, 0))},
            TokType::Newline => { return Node::new(Token::new(TokType::Newline, String::new(), Literal::Null, 0)); },
            _ => panic!("unrecognized token type '{:?}' while parsing", t.ltype),
        }
    }

    fn parse_add(&mut self) -> Node {
        let mut a = Node::new(Token::new(TokType::Add, String::new(), Literal::Null, self.toks[self.tokidx].line));
        while self.advance().ltype != TokType::Newline {
            a.children.push(self.parse_expr());
        }

        a
    }

    fn advance(&mut self) -> Token {
        let b = &self.toks.clone()[self.tokidx];
        self.tokidx += 1;
        return b.clone();
    }
}
