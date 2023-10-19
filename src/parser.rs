use crate::token::*;
#[derive(Clone, Debug)]
pub struct Node {
    tokval: Token,
    subnodes: Vec<Node>,
}

impl Node {
    pub fn new(tok: Token) -> Self {
        Self {
            tokval: tok,
            subnodes: Vec::new(),
        }
    }
}

pub enum Prog {
    Add(Box<Prog>),
    Sub(Box<Prog>),
    Mul(Box<Prog>),
    Div(Box<Prog>),

    Str(String),
    Num(i64),
    Float(f64),

    Def {
        name: String,
        typ: Literal,
        val: Box<Prog>,
    },

    Block(Vec<Prog>),
    Expr(Vec<Prog>),
    Call(String, Vec<Prog>),
}

pub struct Parser {
    toks: Vec<Token>,

    start: u64,
    current: u64,
    line: u64,

    nodes: Vec<Node>,
    is_at_end: bool,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            toks: tokens,

            start: 0,
            current: 0,
            line: 1,

            nodes: Vec::new(),
            is_at_end: false,
        }
    }

    pub fn eval(&mut self, e: &Expr) {
        match e {
            
            _ => {}
        }
    }

    pub fn parse_nodes(&mut self) -> Vec<Node> {
        while !self.is_at_end {
            if self.toks[self.current as usize].ltype == TokType::Eof {
                break;
            }
            self.start = self.current;
            self.parse_node();
        }

        return self.nodes.clone();
    }

    fn parse_node(&mut self) {
        let n = self.advance();

        match n.ltype {
            TokType::lBrack => self.parse_expr(),
            // TokType::lCurl => self.parse_block(),
            // TokType::lParen => self.parse_arr(),
            _ => {}
        }
    }

    fn parse_expr(&mut self) {
        self.add_node_t(TokType::Expr);
        let len = self.node_len() - 1;

        while self.toks[self.current as usize].ltype != TokType::rBrack {
            if self.toks[self.current as usize].ltype == TokType::Eof {
                break;
            }

            self.nodes[len]
                .subnodes.push(Node::new(self.toks[self.current as usize].clone()));

            println!("{:#?}", self.toks[self.current as usize]);
            
            self.advance();
        }
    }

    fn parse_block() {}
    fn parse_arr() {}

    fn peek(&mut self) -> Token {
        self.toks[(self.current + 1) as usize].clone()
    }

    fn previous(&mut self) -> Token {
        self.toks[(self.current - 1) as usize].clone()
    }

    fn advance(&mut self) -> Token {
        let b = self.toks[self.current as usize].clone();
        self.current += 1;
        return b;
    }

    fn add_node(&mut self, t: Token) {
        self.nodes.push(Node::new(t));
    }

    fn add_node_t(&mut self, tt: TokType) {
        let t = Token::new(tt, String::new(), Literal::Null, self.line);
        self.nodes.push(Node::new(t));
    }

    fn node_len(&mut self) -> usize {
        self.nodes.clone().len()
    }


}