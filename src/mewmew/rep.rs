use crate::mewmew::scanner::Token;


pub enum Stmt{
    
    Asignment( Token , Box<Expr> ),     // Token[ID] = Expr
    Print( Box<Expr> ),                 // ::Expr
    Scan(Token),
    If(Expr , Box<Stmt> , Option<Box<Stmt>>),
    Loop(Expr , Box<Stmt>),
    Stmts(Box<Stmt> , Option<Box<Stmt>>),


}

pub enum Expr{
    
    Abs(Box<Expr>),
    Uminus(Box<Expr>),
    Scratch(Box<Expr>),
    Pow(Box<Expr>),
    Division(Box<Expr> , Box<Expr>),
    Multiplication(Box<Expr> , Box<Expr>),
    Substraction(Box<Expr> , Box<Expr>),
    Addition(Box<Expr> , Box<Expr>),
    Modulas(Box<Expr> , Box<Expr>),
    LessThan(Box<Expr> , Box<Expr>),
    Leftshift(Box<Expr> , Box<Expr>),
    GreaterThan(Box<Expr> , Box<Expr>),
    RightShift(Box<Expr> , Box<Expr>),
    LessThanEqual(Box<Expr> , Box<Expr>),
    GreaterThanEqual(Box<Expr> , Box<Expr>),
    EqualEqual(Box<Expr> , Box<Expr>),
    NotEqual(Box<Expr> , Box<Expr>),
    BoolAnd(Box<Expr> , Box<Expr>),
    BoolOr(Box<Expr> , Box<Expr>),
    ExprBlock(Box<Expr>),
    Mewnum(Token),
    Identifier(Token)


    


}
