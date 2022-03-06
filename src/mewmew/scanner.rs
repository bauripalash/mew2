use std::{process::exit, iter::Peekable};
use std::fmt;
/// Scanner Object
/// * source_char_list -> Vector list of each Characters in Source File/String
/// * Line -> Current line being read
/// * current -> Current index of the Character being processed
/// * token_list -> Vector List of Already processed tokens

#[derive(Debug , PartialEq , Clone, Copy)]
pub enum TokenType {
    EQUAL,      // =
    SEMICOLON,  // ;
    ADD,        // +
    SUB,        // -
    MUL,        // *
    DIV,        // /
    POW,        // **
    ABS,        // ~
    GT,         // >
    GTEQUAL,    // >=
    LT,         // <
    LTEQUAL,    // <=
    EQEQ,       // ==
    NOT,        // !
    NEQ,        // !=
    AND,        // &
    OR,         // |
    MOD,        // %
    COLON,      // :
    PRINTNUM,   // ::
    PRINTCHAR,  // :::
    MEWNUM,     // mew...
    IDENTIFIER, // m.. | _m... | _...
    SCRATCH,    // #....
    ATLOOP,     // @
    QU,         // ?
    COMMA,      // ,
    LBRAC,      // (
    RBRAC,      // )
    EOF,

    OTHER,
}

impl fmt::Display for Token{
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result{
        
        write!(f , "T[{:?} ~> {} <{},{}>]" , self.t_type , self.lexeme , self.line , self.pos)

    }


}


#[derive(Debug , Clone)]
pub struct Token {
    pub t_type: TokenType,
    pub lexeme: String,
    pub pos: usize,
    pub line: usize,
}

pub fn new_token(t_type: TokenType, lexeme: String, pos: usize, line: usize) -> Token {
    let _t: Token = Token {
        t_type,
        lexeme,
        pos,
        line,
    };

    _t 
}

pub struct Scanner {
    pub source_char_list: Vec<char>,
    pub line: usize,
    pub current: usize,
    pub token_list: Vec<Token>,
    pub mew_start: usize,
    pub id_start: usize,
}

impl Scanner {
    /// Create a new Instance of Scanner
    pub fn new(source_char_list: Vec<char>) -> Scanner {
        Self {
            source_char_list,
            line: 1,
            current: 0,
            token_list: Vec::new(),
            mew_start: 0,
            id_start: 0,
        } 
    }

    /// Increment the `current` index
    /// and return the character at that index

    /// Check if `current` index is EOF or not
    pub fn at_eof(&self) -> bool {
        self.current >= self.source_char_list.len()
    }
    
    pub fn get_token_list(self) -> Vec<Token>{
        self.token_list
    }
    ///Check if `current` + offset is EOF
    pub fn is_peek_eof(&self, offindex: usize) -> bool {
         (self.current + offindex) >= self.source_char_list.len() 
    }

    pub fn print_tokens(&self) {
        println!("Length => {}", &self.token_list.len());
        for item in &self.token_list {
            println!(
                "[{:?} ~> {} ~> {} ~> {}]",
                item.t_type, item.lexeme, item.pos, item.line
            );
        }
    }

    /// Returns the next character after `current`-th without consuming it.
    pub fn peek(&self) -> char {
        if self.at_eof() {
            return '\0';
        }
        self.source_char_list[self.current + 1] 
    }

    /// Returns the `current` + peek_offset-th character without consuming it.
    pub fn peek_at(&self, peek_offset: usize) -> char {
        if self.is_peek_eof(peek_offset) {
            return '\0';
        }

        self.source_char_list[self.current + peek_offset]
    }

    pub fn scan_identifier(&mut self) {
        while self.peek() == 'm' || self.peek() == '_' {
            self.current += 1;
        }
        self.current += 1;
        self.token_list.push(new_token(
            TokenType::IDENTIFIER,
            String::from_iter(self.source_char_list[self.id_start..self.current].iter()),
            self.current,
            self.line,
        ));
    }

    pub fn scan_mew_number(&mut self) {
        loop {
            if self.peek() == 'w' && !self.at_eof() {
                self.current += 1;
                if self.peek_at(2) == 'e' && !self.is_peek_eof(2) {
                    continue;
                }
                break;
            }

            self.current += 1;
        }

        //self.current += 1;
        self.token_list.push(new_token(
            TokenType::MEWNUM,
            String::from_iter(self.source_char_list[self.mew_start..=self.current].iter()),
            self.current,
            self.line,
        ));
    }

    pub fn scan_scratch(&mut self) {
        let _cindex = self.current;
        while self.peek() == '#' {
            self.current += 1
        }

        let _tok = new_token(
            TokenType::SCRATCH,
            String::from_iter(self.source_char_list[_cindex..=self.current].iter()),
            self.current,
            self.line,
        );
        self.token_list.push(_tok);
    }

    pub fn scan_single_char_op(&mut self, c: char) {
        let mut has_two_char: bool = false;
        let mut has_three_char: bool = false;
        let mut _type: TokenType;
        match c {
            '=' => {
                if self.peek() == '=' {
                    has_two_char = true;
                    _type = TokenType::EQEQ;
                    // println!("+++ EQEQ");
                    self.current += 1;
                } else {
                    _type = TokenType::EQUAL;
                    // println!("+++ EQUAL");
                }
            }
            '~' => {
                _type = TokenType::ABS;
            }
            '+' => {
                _type = TokenType::ADD;
            }
            '-' => _type = TokenType::SUB,
            '*' => {
                if self.peek() == '*' {
                    // self.current+=1;
                    has_two_char = true;
                    _type = TokenType::POW;
                    self.current += 1;
                } else {
                    _type = TokenType::MUL;
                }
            }
            ';' => {
                _type = TokenType::SEMICOLON;
            }

            '<' => {
                if self.peek() == '=' {
                    has_two_char = true;
                    _type = TokenType::LTEQUAL;
                    self.current += 1;
                } else {
                    _type = TokenType::LT;
                }
            }
            '>' => {
                if self.peek() == '=' {
                    has_two_char = true;
                    _type = TokenType::GTEQUAL;
                    self.current += 1;
                } else {
                    _type = TokenType::GT;
                }
            }
            '!' => {
                if self.peek() == '=' {
                    has_two_char = true;
                    _type = TokenType::NEQ;
                    self.current += 1;
                } else {
                    _type = TokenType::NOT;
                }
            }
            '&' => _type = TokenType::AND,
            '|' => _type = TokenType::OR,
            '%' => _type = TokenType::MOD,
            '?' => _type = TokenType::QU,
            '@' => _type = TokenType::ATLOOP,
            '(' => _type = TokenType::LBRAC,
            ')' => _type = TokenType::RBRAC,
            ',' => _type = TokenType::COMMA,
            ':' => {
                if self.peek() == ':' {
                    if self.peek_at(2) == ':' {
                        has_three_char = true;
                        _type = TokenType::PRINTCHAR;
                        self.current += 2;
                    } else {
                        has_two_char = true;
                        _type = TokenType::PRINTNUM;
                        self.current += 1;
                    }
                } else {
                    _type = TokenType::COLON;
                }
            }

            '/' => {
                if self.peek() == '/' {
                    self.eat_comment();
                    return;
                } else {
                    _type = TokenType::DIV
                }
            }
            _ => {
                _type = TokenType::OTHER;
                println!("Unexpected Char");
                // exit(1);
            }
        }
        let _char: String = {
            if has_two_char {
                String::from_iter(vec![c, self.source_char_list[self.current]])
            } else if has_three_char {
                String::from_iter(vec![
                    c,
                    self.source_char_list[self.current - 1],
                    self.source_char_list[self.current],
                ])
            } else {
                c.to_string()
            }
        };

        // println!("{}", _char);
        let _tok = new_token(_type, _char, self.current, self.line);
        self.token_list.push(_tok);
        // self.current += 1;
        // self.current+=1;
    }

    pub fn eat_comment(&mut self) {
        let _cindex = self.current;
        loop {
            self.current += 1;
            if self.source_char_list[self.current] == '\n' {
                self.line += 1;
                break;
            }
        }
    }

    //TODO : Just for testing and/or debugging
    pub fn scan_token(&mut self) {
        while !self.at_eof() {
            let _vcur = self.source_char_list[self.current];
            if _vcur != ' ' {
                if _vcur == '\n' {
                    self.line += 1;
                    self.current += 1;
                    continue;
                }
                match &_vcur {
                    '+' | '-' | '*' | '~' | '&' | '|' | '%' | '>' | '<' | '!' | ';' | '=' | ':'
                    | '/' | '(' | ')' | ',' => self.scan_single_char_op(_vcur),
                    '#' => {
                        self.scan_scratch();
                    }
                    'm' => {
                        if self.peek() == 'e' && self.peek_at(2) == 'w' {
                            self.mew_start = self.current;
                            self.scan_mew_number();
                        } else if self.peek() == 'm' || self.peek() == '_' {
                            self.id_start = self.current;
                            self.scan_identifier();
                        } else {
                            let _tok = new_token(
                                TokenType::OTHER,
                                String::from(_vcur),
                                self.current,
                                self.line,
                            );
                            self.token_list.push(_tok);
                        }
                    }
                    _ => self.token_list.push(new_token(
                        TokenType::OTHER,
                        String::from(_vcur),
                        self.current,
                        self.line,
                    )),
                }
            }
            self.current += 1;
        }
    }
}
