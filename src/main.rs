#![allow(dead_code)]
#![allow(unused_imports)]
use crate::mewmew::scanner::*;
use crate::mewmew::parser::*;
use std::{env, fs, os, process::exit};

mod mewmew;

fn lex(c: &str) {
    let mut m_scanner: Scanner = Scanner::new(c.chars().collect::<Vec<char>>());
    m_scanner.scan_token();
    //m_scanner.print_tokens();
    for item in m_scanner.get_token_list(){
        
        println!("{:}" , item);

    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("/ᐠ｡ꞈ｡ᐟ\\");
    println!("~~~~~~~~");
    if args.len() == 2 {
        let filename = &args[1];
        let contents = fs::read_to_string(filename).expect("Error happened");
        lex(&contents);
    } else {
        println!("[-_-]? Please select a Mew Script to run!");
        println!("Do Something like this => `mewmew [FILENAME]`");
        println!("Bye bye");
        exit(1)
    }
}
