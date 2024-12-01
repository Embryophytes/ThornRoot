use super::column::Column;
use super::relationship::Relationship;
use super::schema::Schema;

#[derive(Default)]
pub struct Table {
    name: String,
    columns: Vec<Column>,
}

impl Table {
    pub fn new(table_name: &str) -> Self {
        Self {
            name: table_name.to_string(),
            columns: vec![],
        }
    }

    pub fn get_columns(&self) -> &[Column] {
        &self.columns
    }

    pub fn get_columns_mut(&mut self) -> &mut Vec<Column> {
        &mut self.columns
    }

    pub fn get_column(&self, _column_name: &str) -> Result<&Column, String> {
        todo!();
    }

    pub fn get_column_mut(&mut self, _column_name: &str) -> Result<&mut Column, String> {
        todo!()
    }

    pub fn add_column(&mut self, _column: Column) -> Result<&Column, String> {
        todo!()
    }

    pub fn udpate_column(&mut self, _updated_column: Column) -> Result<&Column, String> {
        todo!()
    }

    pub fn get_relationships<'a>(&self, schema_editor: &'a Schema) -> Vec<&'a Relationship> {
        schema_editor.get_relationships_for_table(&self.name)
    }
}
