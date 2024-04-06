
pub mod field_text;

pub trait Field {
    fn name(&self) -> String;
    fn analyze(&self);
    fn value(&self) -> Vec<u8>;
}

pub struct Document<'a> {
    pub id: String,
    pub fields: Vec<&'a dyn Field>,
}

impl<'a> Document<'a> {
    pub fn new(id: String) -> Self {
        Self { id: id, fields: vec![] }
    }
}