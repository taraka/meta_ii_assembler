use std::io::{self, Write, Read};
use std::str;
use std::collections::HashMap;

struct Parser<'a> {
    text: &'a str,
    labels: HashMap<String, usize>
}


impl<'a> Parser<'a> {
    fn new(text: &str) -> Parser {
        Parser {
            text,
            labels: HashMap::new()
        }
    }

    fn parse(&self) {
        for l in code.lines() {
            if l.chars().nth(0).unwrap() == '\t' {
                parse_instruction(l);
            } else {
                parse_label(l)
            }
        }
    }

    fn parse_label(&self, l: &str) {
        println!("{}", l);
    }

    fn parse_instruction(&self, l: &str) {
        println!("{}", l);
    }
}


fn main() {
    let mut code_bytes: Vec<u8> = Vec::new();
    io::stdin().read_to_end(&mut code_bytes);

    let code = str::from_utf8(&code_bytes[..]).unwrap();
    let parser = Parser::new(&code);
    parser.parse()

}