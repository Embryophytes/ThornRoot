use crate::core::transform_to_api_code;

use super::column::Column;
use super::relationship::Relationship;

#[derive(Default)]
pub struct Table {
    pub name: String,
    pub columns: Vec<Column>,
    pub relationships: Vec<Relationship>,
    pub saved_with_error: bool,
    #[allow(dead_code)]
    api_code: String,
    // I guess that we must store a unique value on each table and column in order to track migrations
    // inside the engine we can store a created entities codes
    // if a new CODE has appeared - this entity must be created using CREATE
    // If this entity has been found in the engine - check if any values has been updated (it must be a cascade one)
    // if some code in the saved entity has not been appeared - this entity must be deleted

    // Delete
    // Update
    // new
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
            api_code: transform_to_api_code(table_name, crate::core::ApiCodeType::Table),
        }
    }

    /// addes column into the table
    pub fn add_column(
        &mut self,
        column: Column,
        _save_with_error: bool,
    ) -> Result<&mut Self, String> {
        // simple check if such a column with such a name has already been created
        let possible_api_code = column.get_api_code();
        let res = self
            .columns
            .iter()
            .position(|t| t.get_api_code() == possible_api_code);
        match res {
            Some(_) => Err(format!(
                "Column with name '{}' is already created",
                column.name.as_str()
            )),
            None => {
                self.columns.push(column);
                Ok(self)
            }
        }
    }

    /// updates the column in the table
    pub fn update_column(
        &mut self,
        updated_column: Column,
        _save_with_error: bool,
    ) -> Result<&mut Self, String> {
        let existing_api_code = updated_column.get_api_code();
        // here also must be check if there are any relations with this table has been found
        // and the type of the column hasn't changed

        let index = self
            .columns
            .iter()
            .position(|t| t.get_api_code() == existing_api_code)
            .ok_or(format!(
                "Column with name '{}' doesn't exist in the table",
                updated_column.name.as_str()
            ))?;
        // i do have a real column name here
        // if there is any relations and the type of the column has changed then return an error
        let existing_relation = self
            .relationships
            .iter()
            .find(|r| r.from_column == existing_api_code || r.to_column == existing_api_code);
        if existing_relation.is_some() && updated_column.data_type != self.columns[index].data_type
        {
            return Err("Remove existing relations before changing data type of the column".to_string());
        }
        let _ = std::mem::replace(&mut self.columns[index], updated_column);
        Ok(self)
    }

    /// Deletes the column in the table
    pub fn delete_column(
        &mut self,
        column_code_to_delete: &str,
        _save_with_error: bool,
    ) -> Result<&mut Self, String> {
        let index = self
            .columns
            .iter()
            .position(|t| t.get_api_code() == column_code_to_delete)
            .ok_or(format!(
                "Column with name '{}' doesn't exist in the table",
                column_code_to_delete
            ))?;
        let existing_relation = self.relationships.iter().find(|r| {
            r.from_column == column_code_to_delete || r.to_column == column_code_to_delete
        });
        if existing_relation.is_some() {
            return Err("Remove existing relations before deleting the column".to_string());
        }
        let _ = self.columns.swap_remove(index);
        Ok(self)
    }

    /// addes a relation into the table
    pub fn add_relation(
        &mut self,
        relationship: Relationship,
        _save_with_error: bool,
    ) -> Result<&mut Self, String> {
        // just check if such a relation is already created for the given table
        let index = self.relationships.iter().position(|r| *r == relationship);
        match index {
            Some(_) => Err("Such a relation is already created for the given tables and columns".to_string()),
            None => {
                self.relationships.push(relationship);
                Ok(self)
            }
        }
    }

    /// updated a relation in the table (TODO)
    pub fn update_relation(
        &mut self,
        _updated_relationship: Relationship,
        _save_with_error: bool,
    ) -> Result<&mut Self, String> {
        // look like we don't need it

        // ask Braza
        Ok(self)
    }

    /// Deletes a relation from the table
    pub fn delete_relation(
        &mut self,
        relationship_to_delete: Relationship,
    ) -> Result<&mut Self, String> {
        let index = self
            .relationships
            .iter()
            .position(|r| *r == relationship_to_delete)
            .ok_or("Such a realation doesn't exist".to_string())?;
        let _ = self.relationships.swap_remove(index);
        Ok(self)
    }
}
