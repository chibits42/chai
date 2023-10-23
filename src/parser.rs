use crate::token::*;

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
    idx: usize,
}

impl Parser {
    pub fn new(toks: Vec<Token>) -> Self {
        Self {
            out: Vec::new(),
            outidx: 0,
            toks: toks,
            idx: 0, 
        }
    }

    pub fn parse(&mut self) -> Vec<Node> {
        println!("{:#?}", self.toks);
        self.toks.push(Token::null());
        while !self.is_at_end() { 
            //println!("{:?}", i);
            let n = self.parse_expr();
            self.out.push(n);

            self.advance();
        }

        self.out.clone()
    }

    pub fn parse_expr(&mut self) -> Node {
        if self.is_at_end() { return Node::null(); }
        //println!("{}", self.tokidx);
        let t = self.get();

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
            TokType::Add => self.parse_arith(TokType::Add),
            TokType::Sub => self.parse_arith(TokType::Sub),
            TokType::Mul => self.parse_arith(TokType::Mul),
            TokType::Div => self.parse_arith(TokType::Div),

            TokType::lBrack => return self.parse_brack(),

            // TokType::Eof => { return Node::new(Token::new(TokType::Eof, String::new(), Literal::Null, 0))},
            // TokType::Newline => { return Node::new(Token::new(TokType::Newline, String::new(), Literal::Null, 0)); },
            _ => { return Node::new(t); }, //panic!("unrecognized token type '{:?}' while parsing", t.ltype),
        }
    }

    // parses +, -, *, /
    fn parse_arith(&mut self, op: TokType) -> Node {
        let mut a = Node::new(Token::new(
            op, String::new(), Literal::Null, 0,
        ));

        self.advance();

        while !self.is_at_end() && self.get().ltype != TokType::rBrack {
            a.children.push(self.parse_expr());
            self.advance();
        }

        a
    }

    fn parse_brack(&mut self) -> Node {
        let mut e = Node::new(Token::new(
            TokType::Expr, String::new(), Literal::Null, 0,
        ));

        self.advance();

        while !self.is_at_end() && self.get().ltype != TokType::rBrack {
            e.children.push(self.parse_expr());
            self.advance();
        }

        e
    }

    fn is_at_end(&self) -> bool {
        return self.idx as usize >= self.toks.len() - 1;
    }

    fn get(&self) -> Token {
        if self.is_at_end() { return self.toks[self.toks.len() - 1].clone(); }
        return self.toks[self.idx].clone();
    }

    fn advance(&mut self) -> Token {
        if self.is_at_end() { return self.toks[self.toks.len() - 1].clone(); }
        self.idx += 1;
        return self.toks[self.idx].clone();

    }
} 
