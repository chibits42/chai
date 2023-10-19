mod token;
mod scanner;
mod parser;

fn main() {
    let src = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();

    let mut scan = scanner::Scanner::new(src);
    let mut toks = scan.scan_tokens();

    let mut parse = parser::Parser::new(toks);
    let mut nodes = parse.parse_nodes();

    println!("{:#?}", nodes);
}
