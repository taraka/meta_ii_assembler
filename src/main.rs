use std::io::{self, Read, Write};
use std::str;
use std::collections::{HashMap, LinkedList};
use std::error::Error;

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
    ADR = 1,
    TST = 2,
    BF = 3,
    ID = 4,
    BE = 5,
    CL = 6,
    CI = 7,
    OUT = 8,
    CLL = 9,
    BT = 10,
    SET = 11,
    R = 12,
    END = 13,
    STR = 14,
    NUM = 15,
    LB = 16,
    GN1 = 17,
    GN2 = 18,
    B = 19,
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
            //println!("Looking for label: {}", lm.label);
            let label_addr = self.labels.get(&lm.label[..]).expect("Undefined label");
            //println!("Address for label: {}, is {}", lm.label, label_addr);
            let mut offset: usize = 0;
            for b in label_addr.to_le_bytes().iter() {
                self.code.insert(lm.addr + offset, *b);
                offset += 1;
            }
        }

        //println!("{:?}", self.code);
    }

    fn parse_label(&mut self, l: &str) {
        self.labels.insert(l.trim().to_string(), self.code.len());
    }

    fn parse_instruction(&mut self, l: &str) {
        let opcode = Parser::opcode(l.trim().split_whitespace().nth(0).unwrap());
        match opcode {
            Opcode::ADR => self.adr(l),
            Opcode::TST => self.tst(l),
            Opcode::BF => self.bf(l),
            Opcode::ID => self.id(),
            Opcode::BE => self.be(),
            Opcode::CL => self.cl(l),
            Opcode::CI => self.ci(),
            Opcode::OUT => self.out(),
            Opcode::CLL => self.cll(l),
            Opcode::BT => self.bt(l),
            Opcode::SET => self.set(),
            Opcode::R => self.r(),
            Opcode::END => self.end(),
            Opcode::STR => self.str(),
            Opcode::NUM => self.num(),
            Opcode::LB => self.lb(),
            Opcode::GN1 => self.gn1(),
            Opcode::GN2 => self.gn2(),
            Opcode::B => self.b(l),
        }
    }

    fn adr(&mut self, l: &str) {
        self.code.push(Opcode::ADR as u8);
        self.write_label(l);
    }

    fn bf(&mut self, l: &str) {
        self.code.push(Opcode::BF as u8);
        self.write_label(l);
    }

    fn tst(&mut self, l: &str) {
        self.code.push(Opcode::TST as u8);
        self.write_string(l);
    }

    fn id(&mut self) {
        self.code.push(Opcode::ID as u8);
    }

    fn be(&mut self) {
        self.code.push(Opcode::BE as u8);
    }

    fn ci(&mut self) {
        self.code.push(Opcode::CI as u8);
    }

    fn out(&mut self) {
        self.code.push(Opcode::OUT as u8);
    }

    fn cl(&mut self, l: &str) {
        self.code.push(Opcode::CL as u8);
        self.write_string(l);
    }

    fn cll(&mut self, l: &str) {
        self.code.push(Opcode::CLL as u8);
        self.write_label(l);
    }

    fn bt(&mut self, l: &str) {
        self.code.push(Opcode::BT as u8);
        self.write_label(l);
    }

    fn set(&mut self) {
        self.code.push(Opcode::SET as u8);
    }

    fn r(&mut self) {
        self.code.push(Opcode::R as u8);
    }

    fn end(&mut self) {
        self.code.push(Opcode::END as u8);
    }

    fn str(&mut self) {
        self.code.push(Opcode::STR as u8);
    }

    fn num(&mut self) {
        self.code.push(Opcode::NUM as u8);
    }

    fn lb(&mut self) {
        self.code.push(Opcode::LB as u8);
    }

    fn gn1(&mut self) {
        self.code.push(Opcode::GN1 as u8);
    }

    fn gn2(&mut self) {
        self.code.push(Opcode::GN2 as u8);
    }

    fn write_label(&mut self, line: &str) {
        self.add_label_addr(line.trim().split_whitespace().nth(1).unwrap());
    }

    fn b(&mut self, l: &str) {
        self.code.push(Opcode::B as u8);
        self.write_label(l);
    }

    fn write_string(&mut self, line: &str) {

        let mut in_string = false;

        for c in  line.as_bytes() {
            //String char
            if in_string && *c != b'\'' {
                self.code.push(*c);
            }

            // End of string
            if in_string && *c == b'\'' {
                self.code.push(0);
                break;
            }

            // Start of string
            if !in_string && *c == b'\'' {
                in_string = true;
            }
        }
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

    /// Header format
    /// XXXXXX Magic bytes ".meta"
    /// X version (Currently 0)
    /// X Endienness 0 for LE
    /// X addr size
    fn header(&self) -> [u8; 8] {


        let addrsize = std::mem::size_of::<usize>() as u8;

        [46,109,101,116,97,0,0, addrsize]
    }

    fn opcode(text: &str) -> Opcode {
        match text {
            "ADR" => Opcode::ADR,
            "TST" => Opcode::TST,
            "BF" => Opcode::BF,
            "ID" => Opcode::ID,
            "BE" => Opcode::BE,
            "CL" => Opcode::CL,
            "CI" => Opcode::CI,
            "OUT" => Opcode::OUT,
            "CLL" => Opcode::CLL,
            "BT" => Opcode::BT,
            "SET" => Opcode::SET,
            "R" => Opcode::R,
            "END" => Opcode::END,
            "STR" => Opcode::STR,
            "NUM" => Opcode::NUM,
            "LB" => Opcode::LB,
            "GN1" => Opcode::GN1,
            "GN2" => Opcode::GN2,
            _ => panic!(format!("Unknown opcode: {}", text))
        }
    }
}



fn main() -> Result<(), Box<dyn Error>> {
    let mut code_bytes: Vec<u8> = Vec::new();
    io::stdin().read_to_end(&mut code_bytes)?;

    let code = str::from_utf8(&code_bytes[..]).unwrap();
    let mut parser = Parser::new(&code);
    parser.parse();

    // println!("{:?}", parser.header());
    // println!("{:?}", parser.code);

    // TODO: Need to add a program headers including info about address sizes and endinenness
    io::stdout().write_all(&parser.header()[..])?;
    io::stdout().write_all(&parser.code[..])?;

    Ok(())
}