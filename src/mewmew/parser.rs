use std::usize;

use crate::mewmew::rep::*;
use crate::mewmew::scanner::*;


pub struct Parser{
    
    pub tokens : Vec<Token>,
    pub current : usize,
    pub stmt_list : Vec<Stmt>

}

impl Parser{

    pub fn new(tokens : Vec<Token>) -> Parser{
        
        Parser{
            
            tokens,
            current : 0,
            stmt_list : Vec::new()

        }

    }

    pub fn at_eof(&self) -> bool{
        
        self.tokens.len() <= self.current

    }

    pub fn at_peek_eof(&self , x : usize) -> bool{
        
        self.tokens.len() <= x

    }

    pub fn peek(&self) -> &Token{
         &self.tokens[self.current + 1]

    }

    pub fn backpeek(&self) -> &Token{
        
        &self.tokens[self.current - 1]

    }

    pub fn advance(&mut self) -> &Token{
        
        if !self.at_eof(){
            
            self.current += 1;

        }

        self.backpeek()

    }

    pub fn check(&self , token_type : &TokenType) -> bool{
        
        if self.at_eof(){
           return false; 
        }
        //println!("PEEK => {:?} | C => {:?}" , self.peek().t_type , *token_type);
        self.peek().t_type == *token_type

    }

    pub fn nexts(&mut self , types : &[TokenType]) -> bool{
        let mut out = false;
        let old_current = self.current.clone();
        for token_type in types{
            
            if self.check(token_type){
                
                //self.advance();

                out = true;
                self.current += 1;
            }
            else{
                out=false;
            }

        }
        self.current = old_current;
        out

        

    }

    pub fn eat(&mut self , tok_type : TokenType , err_msg : &str) -> Token{
        
        if self.check(&tok_type){
            self.advance();
            let o =  self.advance().clone();
            println!("Ate => {:?}" , o);
            o
        }else{
            
            //let err_tok = self.peek();
            panic!("Parser Error => {}" , err_msg);

        }

    }

    pub fn scan(&mut self){
        //let mut stmts : Vec<Stmt> = Vec::new();
        self.declration()
    }

    pub fn declration(&mut self){
        if self.tokens[self.current].t_type == TokenType::IDENTIFIER{ 
            if self.nexts(&[TokenType::EQUAL]){
                //println!("=>>{}<<=" , self.peek());
                let id = &self.tokens[self.current];
                println!("ID => {:?}" , id);
                self.advance();
                let expr = self.expression(); 
                //let x = Stmt::Asignment(id , Box::new(Expr::Mewnum(expr)));
                //println!("Expr =>");
            }
        }
    

    }

    pub fn expression(&mut self) -> bool{
        let expr = self.addition();
        if self.nexts(&[TokenType::MEWNUM]){
             
            println!("Mewnum assignment");
            let x = self.eat(TokenType::MEWNUM, "Expected MewNum");
            let o = Expr::Mewnum(x);
            println!("{:?}" , o);

        }
    
       false 

    }

    pub fn addition(&mut self) -> bool{
       
        if self.nexts(&[TokenType::ADD]){
            
            

        }
        false

    }




}
