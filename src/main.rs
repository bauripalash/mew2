use std::{fs, str::Chars, char, error::Error};



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

// enum TokenType{
    // ID,
    // INTEGER
// }

// struct Token{
    
    // ttype : String,
    // lexeme : String,
    // line : u32

// }

// impl Token{
    // fn to_string(&self) -> String{
        // let s = String::from(format!("[Token][Type => {} ; Lexeme => '{}' ; Line => {}]" , self.ttype , self.lexeme , self.line));
        // return s;
    // }
// }


// struct Scanner{
    
    // source : String,
    // tokens : Vec<Token>,
    // start : u32,
    // current : u32,
    // line : u32 

// }

// impl Scanner{
    // fn is_at_end(&self) -> bool{
        // return self.current >= self.source.len() as u32; 
    // }
    // fn scan_token(&mut self){
        // while !self.is_at_end(){
            // self.start = self.current;
            // self.scan_token();
        // }
        
        // self.tokens.push(Token{ ttype : String::from("EOF") , lexeme : String::from("") , line : 1 });
    // }
    // fn pnt(&self){
        // for item in self.source.chars(){
            // println!("{}" , item.to_string())
        // }
    // }
// }

struct Token{
    lexeme : String,
    pos : u32,
    line : u32
}

fn lex(c : String){
//    let mut cx : Vec<Chars> = vec![*c];
    // let length = c.len();
    // let mut current : u32 = 0;
    // let mut start : u32 = 0;
    // let mut line : u32 = 1;
//     let mut s_obj : Scanner = Scanner {
        // source : c,
        // tokens : Vec::new(),
        // start : 0,
        // current : 0,
        // line : 1
    // };
    // s_obj.scan_token();
//     s_obj.pnt();
    let mut current : u32 = 0;
    let mut line : u32 = 1;
    let mut token_list : Vec<Token> = Vec::new();
    let _vc = c.chars().collect::<Vec<char>>();
     while !(current >= _vc.len() as u32) {
         let vcur = _vc[current as usize];
        // if c[current as usize] != ' ' || c[current as usize] != '\n'{
        // println!("=> {} <=" , c[current as usize]);
        // current+=1
        if !(vcur ==  ' '){
        if vcur == '\n'{
           line += 1; 
           current+=1;
           continue;
        }
        let _token : Token = Token{
            lexeme : String::from(vcur),
            pos : current,
            line
        };

        token_list.push(_token);

        // println!("{:?}" , _token);
        
        
        
        }
        current += 1;
        
     }

     println!("+ Length => {}" , token_list.len());
     for item in &token_list{
         println!("[Token ~> {} ~> {} ~> {}]" , item.lexeme , item.pos , item.line);
     }

}

fn main() {
    let _f = String::from("/home/palash/Desktop/rol/c/src/a.txt");

    let _fc = fs::read_to_string(_f).unwrap();
    // let ic = _fc.chars().collect::<Vec<char>>();
    lex(_fc);
    //for item in ic{
    //    println!("{}" , item);
    //}
    // println!("{}" , _fc);
    // iter_string(_fc);

}
