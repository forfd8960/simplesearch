use crate::{analysis, index};


pub struct TextField<'a> {
    name: String,
    array_pos: Vec<usize>,
    analyzer: &'a dyn analysis::Analyzer,
    value: Vec<u8>,
    length: i32,
    frequencies: index::TokenFrequency,
}