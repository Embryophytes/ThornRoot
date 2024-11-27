use super::column::Column;
use super::relationship::Relationship;

#[derive(Default)]
pub struct Table {
    pub name: String,
    pub columns: Vec<Column>,
    pub relationships: Vec<Relationship>,
    pub saved_with_error: bool,
}

impl Drop for Table {
    fn drop(&mut self) {
        let relationships = std::mem::take(&mut self.relationships);

        for relationship in relationships {
            let _ = self.delete_relation(relationship);
        }
    }
}

impl Table {
    pub fn new(table_name: &str) -> Self {
        Self {
            name: table_name.to_string(),
            columns: vec![],
            relationships: vec![],
            saved_with_error: false,
        }
    }

    /// addes column into the table (TODO)
    pub fn add_column(
        &mut self,
        _column: Column,
        _save_with_error: bool,
    ) -> Result<&mut Self, String> {
        Ok(self)
    }

    /// updates the column in the table (TODO)
    pub fn update_column(
        &mut self,
        _updated_column: Column,
        _save_with_error: bool,
    ) -> Result<&mut Self, String> {
        Ok(self)
    }

    /// Deletes the column in the table (TODO)
    pub fn delete_column(
        &mut self,
        _column_name_to_delete: &str,
        _save_with_error: bool,
    ) -> Result<&mut Self, String> {
        Ok(self)
    }

    /// addes a relation into the table (TODO)
    pub fn add_relation(
        &mut self,
        _relationship: Relationship,
        _save_with_error: bool,
    ) -> Result<&mut Self, String> {
        Ok(self)
    }

    /// updated a relation in the table (TODO)
    pub fn update_relation(
        &mut self,
        _updated_relationship: Relationship,
        _save_with_error: bool,
    ) -> Result<&mut Self, String> {
        Ok(self)
    }

    /// Deletes a relation from the table (TODO)
    pub fn delete_relation(
        &mut self,
        _relationship_to_delete: Relationship,
    ) -> Result<&mut Self, String> {
        Ok(self)
    }
}
