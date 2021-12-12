#![allow(dead_code)]
#![allow(unused_imports)]
use crate::mewmew::scanner::*;
use std::fs;
use std::os;

mod mewmew;

fn lex(c: &String) {
    let mut m_scanner: Scanner = Scanner::new(c.chars().collect::<Vec<char>>());
    m_scanner.scan_token();
    //m_scanner.print_tokens();
}

fn main() {
    let _f : String = String::from("/home/palash/Desktop/rol/c/src/a.txt");
    let _fc = fs::read_to_string(_f).unwrap();
    lex(&_fc);
}
