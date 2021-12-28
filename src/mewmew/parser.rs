use std::usize;

use crate::mewmew::rep::*;
use crate::mewmew::scanner::*;


pub struct Parser{
    
    pub tokens : Vec<Token>,
    pub current : usize

}

impl Parser{

    pub fn new(tokens : Vec<Token>) -> Parser{
        
        Parser{
            
            tokens,
            current : 0,

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


}
