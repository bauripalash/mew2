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

}
