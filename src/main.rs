use std::io::{self, Read};
use std::str;
use std::collections::{HashMap, LinkedList};

struct Parser<'a> {
    text: &'a str,
    labels: HashMap<String, usize>,
    label_markers: LinkedList<LabelMarker>,
    code: Vec<u8>
}

struct LabelMarker {
    label: String,
    addr: usize
}

enum Opcode {
    ADR = 1
}

impl<'a> Parser<'a> {
    fn new(text: &str) -> Parser {
        Parser {
            text,
            labels: HashMap::new(),
            label_markers: LinkedList::new(),
            code: Vec::new()
        }
    }

    fn parse(&mut self) {
        for l in self.text.lines() {
            if l.chars().nth(0).unwrap() == '\t' {
                self.parse_instruction(l);
            } else {
                self.parse_label(l)
            }
        }

        for lm in &self.label_markers {
            let label_addr = self.labels.get(&lm.label[..]).expect("Undefined label");

            let mut offset: usize = 0;
            for b in label_addr.to_le_bytes().iter() {
                self.code.insert(lm.addr + offset, *b);
                offset += 1;
            }
        }
    }

    fn parse_label(&mut self, l: &str) {
        self.labels.insert(l.trim().to_string(), self.code.len());
    }

    fn parse_instruction(&mut self, l: &str) {
        let opcode = Parser::opcode(l.trim().split_whitespace().nth(0).unwrap());
        match opcode {
            Opcode::ADR => self.adr(l)
        }
    }

    fn adr(&mut self, l: &str) {
        self.code.push(Opcode::ADR as u8);
        self.add_label_addr(l.trim().split_whitespace().nth(1).unwrap());
    }

    fn add_label_addr(&mut self, label: &str) {
        let tmp_addr: usize = 0;

        self.label_markers.push_back(LabelMarker {
            label: label.to_string(),
            addr: self.code.len()
        });

        for b in tmp_addr.to_le_bytes().iter() {
            self.code.push(*b);
        }

    }

    fn opcode(text: &str) -> Opcode {
        match text {
            "ADR" => Opcode::ADR,
            _ => panic!(format!("Unknown opcode: {}", text))
        }
    }
}



fn main() {
    let mut code_bytes: Vec<u8> = Vec::new();
    io::stdin().read_to_end(&mut code_bytes);

    let code = str::from_utf8(&code_bytes[..]).unwrap();
    let mut parser = Parser::new(&code);
    parser.parse()

}