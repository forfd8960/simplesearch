use std::collections::HashMap;

pub struct TokenLocation {
	pub field: String,
    pub array_positions: Vec<u64>,
    pub start: i32,
    pub end: i32,
    pub pos: i32,
}

pub struct TokenFreq {
    pub term: Vec<u8>,
    pub locations: Vec<TokenLocation>,
    pub frequency: i32,
}

pub type TokenFrequency = HashMap<String, TokenFreq>; 