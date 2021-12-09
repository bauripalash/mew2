#![allow(dead_code)]
use std::fs;

struct Token {
    lexeme: String,
    pos: u32,
    line: u32,
}

fn new_token(lexeme: String, pos: u32, line: u32) -> Token {
    let _t: Token = Token { lexeme, pos, line };

    return _t;
}

struct Scanner {
    c_list: Vec<char>,
    line: u32,
    current: usize,
    tok_list: Vec<Token>,
}

impl Scanner {
    fn c_advance(mut self) -> char {
        self.current += 1;

        return self.c_list[self.current - 1];
    }
    fn at_eof(&self) -> bool {
        return self.current >= self.c_list.len();
    }
    //TODO : Complete this function
    //Donot use the print_token
    fn scan_token(mut self) {
        while !self.at_eof() {
            let _vcur = self.c_list[self.current];
            if !(_vcur == ' ') {
                if _vcur == '\n' {
                    self.line += 1;
                    self.current += 1;
                    continue;
                }

                let _token: Token = Token {
                    lexeme: String::from(_vcur),
                    pos: self.current as u32,
                    line: self.line,
                };
                self.tok_list.push(_token);
            }
            self.current += 1;
        }
        println!("Scan Token");
    }
    //TODO : Just for testing and/or debugging
    fn print_token(mut self) {
        while !self.at_eof() {
            let _vcur = self.c_list[self.current];
            if !(_vcur == ' ') {
                if _vcur == '\n' {
                    self.line += 1;
                    self.current += 1;
                    continue;
                }
                match &_vcur {
                    '=' => self.tok_list.push(new_token(
                        String::from("EQ"),
                        self.current as u32,
                        self.line,
                    )),
                    ';' => self.tok_list.push(new_token(
                        String::from("SQ"),
                        self.current as u32,
                        self.line,
                    )),
                    _ => self.tok_list.push(new_token(
                        String::from(_vcur),
                        self.current as u32,
                        self.line,
                    )),
                }
            }
            self.current += 1;
        }
        println!("Length => {}", self.tok_list.len());
        for item in &self.tok_list {
            println!(
                "[Token ~> {} ~> {} ~> {}]",
                item.lexeme, item.pos, item.line
            );
        }
    }
}

fn lex(c: String) {
    //     let mut current: u32 = 0;
    // let mut line: u32 = 1;
    // let mut token_list: Vec<Token> = Vec::new();
    // let _vc = c.chars().collect::<Vec<char>>();
    // while !(current >= _vc.len() as u32) {
    // let vcur = _vc[current as usize];
    // if !(vcur == ' ') {
    // if vcur == '\n' {
    // line += 1;
    // current += 1;
    // continue;
    // }
    // let _token: Token = Token {
    // lexeme: String::from(vcur),
    // pos: current,
    // line,
    // };

    // token_list.push(_token);
    // }
    // current += 1;
    // }

    // println!("+ Length => {}", token_list.len());
    // for item in &token_list {
    // println!(
    // "[Token ~> {} ~> {} ~> {}]",
    // item.lexeme, item.pos, item.line
    // );
    // }
    let m_scanner: Scanner = Scanner {
        c_list: c.chars().collect::<Vec<char>>(),
        line: 1,
        current: 0,
        tok_list: Vec::new(),
    };
    m_scanner.print_token();
}

fn main() {
    let _f = String::from("/home/palash/Desktop/rol/c/src/a.txt");

    let _fc = fs::read_to_string(_f).unwrap();
    lex(_fc);
}
