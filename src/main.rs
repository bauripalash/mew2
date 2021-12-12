#![allow(dead_code)]
#![allow(unused_imports)]
use crate::mewmew::scanner::*;
use std::fs;
use std::io::Read;
use std::os;
use std::env;
use std::path;
use std::process::exit;

mod mewmew;

fn lex(c: &String) {
    let mut m_scanner: Scanner = Scanner::new(c.chars().collect::<Vec<char>>());
    m_scanner.scan_token();
    m_scanner.print_tokens();
}

fn main() {
    let args:Vec<_> = env::args().collect();
    let mut _fc : String = String::new();
    println!("/ᐠ｡ꞈ｡ᐟ\\");
    if args.len() < 2{

        println!("Usage: ./mewmew [FILENAME]");
        exit(1);

    }
    else if args.len() == 2 {
        let file_path = path::PathBuf::from(&args[1]);
        let _fc = fs::read_to_string(file_path).unwrap();
        lex(&_fc);
    }
}
