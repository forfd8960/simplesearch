pub mod token;
pub mod tokenizer;
pub mod analyzer;


pub enum TokenType {
    AlphaNumeric,
    Numeric,
}


pub struct Token {
    start: i32,
    end: i32,
    term: Vec<u8>,
    pos: i32, // position in the field
    tk_type: TokenType,
    is_keyword: bool
}

pub trait CharFitler {
     fn filter(&self, tokens: &[u8]) -> Vec<u8>;
}


pub trait TokenFilter {
    fn filter(&self, tokens: &[Token]) -> Vec<Token>;
}

pub trait Analyzer {
    fn analyze(&self, tokens: &[Token]) -> Vec<Token>;
}