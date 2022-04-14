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

    fn eof(&self) -> bool{
        
        if self.tokens[self.current].t_type != TokenType::EOF{
            return false;
        }
        true
    
    }

    fn current_token_type(&self) -> TokenType{
        self.current_token().t_type
    }

    fn current_token(&self) -> Token{
        self.tokens[self.current].clone()
    }

    fn advance(&mut self){
        self.current += 1;
    }

    fn next_token(&self) -> &Token{
        &self.tokens[self.current + 1]
    }

    fn parse_assignment(&mut self){
        let id = self.current_token();
        self.advance();
        let value = self.parse_expression();
        println!("parsing assignment | {} = {}" , id , value);
    }

    fn parse_printnum(&mut self){
        println!("parsing printnum");
    }

    fn parse_expression(&mut self) -> String{
        while self.current_token_type() != TokenType::SEMICOLON{
            self.advance();
        }
        println!("parsing expression");
        String::from("EXPRESSION")
    }

    pub fn parse(&mut self){
            
        while !self.eof(){
            
            println!("{:?}" , self.current_token());
            if self.current_token_type() == TokenType::IDENTIFIER && self.next_token().t_type == TokenType::EQUAL{
                
                self.parse_assignment();

            }else if self.current_token_type() == TokenType::PRINTNUM{
                self.parse_printnum();
            }
            self.current += 1;
        
        }
    
    }

    
}
