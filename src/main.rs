use std::{fs, str::Chars, char};



// fn read_file(f_path : &String) -> String{
    // //println!("{}" , f_path);
    // let f_contents = String::new();
    // let _file = fs::read_to_string(f_path).unwrap();
    // return f_contents;
// }

// fn iter_string(s : String){
    // println!("{}" , s);
    // //for item in {
    // //    println!("=> {}" , item);
    // //}

// }

enum TokenType{
    ID,
    INTEGER
}

fn is_at_eof(length:usize , cur:u32) -> bool{
    return cur >= length as u32;
}

fn lex(c : Vec<char>){
//    let mut cx : Vec<Chars> = vec![*c];
    let length = c.len();
    let mut current : u32 = 0;
    let mut start : u32 = 0;
    let mut line : u32 = 1;
    while !is_at_eof(length , current) {
        if c[current as usize] != ' ' || c[current as usize] != '\n'{
        println!("=> {} <=" , c[current as usize]);
        current+=1;
        
        
    }

}

}

fn main() {
    let _f = String::from("/home/palash/Desktop/rol/c/src/a.txt");

    let _fc = fs::read_to_string(_f).unwrap();
    let ic = _fc.chars().collect::<Vec<char>>();
    lex(ic);
    //for item in ic{
    //    println!("{}" , item);
    //}
    // println!("{}" , _fc);
    // iter_string(_fc);

}
