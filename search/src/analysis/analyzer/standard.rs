use crate::analysis::{Analyzer, CharFitler, TokenFilter, Token};


pub struct StandardAnalyzer<'a> {
    char_filter: Vec<&'a dyn CharFitler>,
    token_filter: Vec<&'a dyn TokenFilter>,
}

impl<'a> StandardAnalyzer<'a> {
    pub fn new(char_fitler: Vec<&'a dyn CharFitler>, token_filter: Vec<&'a dyn TokenFilter>) -> Self {
        StandardAnalyzer{
            char_filter: char_fitler,
            token_filter: token_filter,
        }
    }
}

impl<'a> Analyzer for StandardAnalyzer<'a> {
    fn analyze(&self, tokens: &[Token]) -> Vec<Token> {
        todo!()
    }
}