#![allow(dead_code)]
#![allow(unused_imports)]
use crate::mewmew::scanner::*;
use std::fs;
use std::os;

mod mewmew;

#[derive(Debug)]
enum TokenType {
    EQUAL,
    SEMICOLON,
    MEWNUM,
    OTHER,
}

struct Token {
    t_type: TokenType,
    lexeme: String,
    pos: u32,
    line: u32,
}

fn new_token(t_type: TokenType, lexeme: String, pos: u32, line: u32) -> Token {
    let _t: Token = Token {
        t_type,
        lexeme,
        pos,
        line,
    };

    return _t;
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
    // m_scanner.print_token();
    //
    let m_scanner: Scanner = Scanner::new(c.chars().collect::<Vec<char>>());
    m_scanner.print_token();
}

fn main() {
    let _f = String::from("/home/palash/Desktop/rol/c/src/a.txt");

    let _fc = fs::read_to_string(_f).unwrap();
    lex(_fc);
}
