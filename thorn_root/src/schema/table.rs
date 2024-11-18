use super::column::Column;
use super::relationship::Relationship;

#[derive(Default)]
pub struct Table {
    pub name: String,
    pub columns: Vec<Column>,
    pub relationships: Vec<Relationship>,
}

impl Table {
    pub fn new(table_name: &str) -> Self {
        Self {
            name: table_name.to_string(),
            columns: vec![],
            relationships: vec![]
        }
    }

    pub fn add_column(&mut self, _column: Column) {
        todo!()
    }
}
