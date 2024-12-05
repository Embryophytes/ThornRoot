use super::column::Column;
use super::relationship::Relationship;
use super::Schema;

#[derive(Debug, Default)]
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

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn set_name(&mut self, new_table_name: &str) {
        self.name = new_table_name.to_string();
    }

    pub fn get_columns(&self) -> &[Column] {
        &self.columns
    }

    pub fn get_columns_mut(&mut self) -> &mut Vec<Column> {
        &mut self.columns
    }

    pub fn get_column(&self, column_name: &str) -> Result<&Column, String> {
        match self.columns.iter().find(|c| c.get_name() == column_name) {
            Some(column) => Ok(column),
            None => Err(format!(
                "Column {} doesn't not exist in table {}",
                column_name, self.name
            )),
        }
    }

    pub fn get_column_mut(&mut self, _column_name: &str) -> Result<&mut Column, String> {
        todo!()
    }

    pub fn add_column(&mut self, column: Column) -> Result<&Column, String> {
        self.columns.push(column);
        Ok(self.columns.last().unwrap())
    }

    pub fn udpate_column(&mut self, _updated_column: Column) -> Result<&Column, String> {
        todo!()
    }

    pub fn get_relationships<'a>(&self, schema_editor: &'a Schema) -> Vec<&'a Relationship> {
        schema_editor
            .get_relationships_for_table(&self.name)
            .unwrap()
    }
}
