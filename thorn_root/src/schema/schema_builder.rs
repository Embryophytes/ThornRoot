use std::sync::Arc;

use crate::database::engine::DatabaseEngine;

use super::column::Column;
use super::relationship::Relationship;
use super::table::Table;

// looks like we also need to find the way to save the SchemaBuilder in the file
// oh God no, amazon ion
pub struct SchemaBuilder<T>
where
    T: DatabaseEngine,
{
    tables: Vec<Table>,
    database_engine: Arc<T>,
}

impl<T> Default for SchemaBuilder<T>
where
    T: DatabaseEngine,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> SchemaBuilder<T>
where
    T: DatabaseEngine,
{
    /// Creates a new, empty `SchemaBuilder`.
    pub fn new() -> Self {
        Self {
            tables: Vec::new(),
            database_engine: Arc::default(),
        }
    }

    /// Adds a new table to the schema.
    pub fn add_table(&mut self, table_name: &str) -> &mut Table {
        let table = Table::new(table_name);
        self.tables.push(table);
        self.tables.last_mut().unwrap()
    }

    /// Adds a column to an existing table.
    pub fn add_column(
        &mut self,
        table_name: &str,
        column: Column,
        save_with_error: bool,
    ) -> Result<&mut Table, String> {
        match self.tables.iter_mut().find(|t| t.name == table_name) {
            Some(table) => 
                // save with error means if there is a column with such a name
                // we would still save it but the migration will fail
                table.add_column(column, save_with_error),
            
            None => Err(format!("Table '{}' not found", table_name)),
        }
    }

    /// Updates a column in an existing table.
    pub fn update_column(
        &mut self,
        table_name: &str,
        updated_column: Column,
        save_with_error: bool,
    ) -> Result<&mut Table, String> {
        match self.tables.iter_mut().find(|t| t.name == table_name) {
            Some(table) => 
                // save with error means if there is a column with such a name
                // we would still save it but the migration will fail
                table.update_column(updated_column, save_with_error),
            
            None => Err(format!("Table '{}' not found", table_name)),
        }
    }

    /// Deletes a column from an existing table.
    pub fn delete_column(
        &mut self,
        table_name: &str,
        column_name_to_delete: &str,
        save_with_error: bool,
    ) -> Result<&mut Table, String> {
        // to make it easier you firstly need to remove all the relations and then you can remove the column
        match self.tables.iter_mut().find(|t| t.name == table_name) {
            Some(table) => 
                // save with error means if there is a column with such a name
                // we would still save it but the migration will fail
                table.delete_column(column_name_to_delete, save_with_error),
            
            None => Err(format!("Table '{}' not found", table_name)),
        }
    }

    /// Add a relation to the schema.
    pub fn add_relation(
        &mut self,
        relationship: Relationship,
        save_with_error: bool,
    ) -> Result<(), String> {
        let from_table = match self.tables.iter_mut().find(|t| t.name == relationship.from_table) {
            Some(table) => table,
            None => return Err(format!("Table '{}' not found", relationship.from_table)),
        };
        from_table.add_relation(relationship.clone(), save_with_error)?;
        let to_table: &mut Table = match self.tables.iter_mut().find(|t| t.name == relationship.to_table) {
            Some(table) => table,
            None => return Err(format!("Table '{}' not found", relationship.to_table)),
        };
        to_table.add_relation(relationship, save_with_error)?;
        Ok(())   
    }

    /// Updates a relation in the schema.
    pub fn update_relation(
        &mut self,
        updated_relationship: Relationship,
        save_with_error: bool,
    ) -> Result<(), String> {
        let from_table = match self.tables.iter_mut().find(|t| t.name == updated_relationship.from_table) {
            Some(table) => table,
            None => return Err(format!("Table '{}' not found", updated_relationship.from_table)),
        };
        from_table.update_relation(updated_relationship.clone(), save_with_error)?;
        let to_table: &mut Table = match self.tables.iter_mut().find(|t| t.name == updated_relationship.to_table) {
            Some(table) => table,
            None => return Err(format!("Table '{}' not found", updated_relationship.to_table)),
        };
        to_table.update_relation(updated_relationship, save_with_error)?;
        Ok(())
    }

    /// Deletes a relation from the schema.
    pub fn delete_relation(
        &mut self,
        relationship_to_delete: Relationship,
    ) -> Result<(), String> {
        let from_table = match self.tables.iter_mut().find(|t| t.name == relationship_to_delete.from_table) {
            Some(table) => table,
            None => return Err(format!("Table '{}' not found", relationship_to_delete.from_table)),
        };
        from_table.delete_relation(relationship_to_delete.clone())?;
        let to_table: &mut Table = match self.tables.iter_mut().find(|t| t.name == relationship_to_delete.to_table) {
            Some(table) => table,
            None => return Err(format!("Table '{}' not found", relationship_to_delete.to_table)),
        };
        to_table.delete_relation(relationship_to_delete)?;
        Ok(())
    }

    /// Builds the schema and returns the result.
    pub fn build(&mut self) -> Result<(), String> {
        // buildes the schema - saves it to the files
        // self.tables
        Ok(())
    }

    pub fn database_engine(&self) -> &T {
        &self.database_engine // not sure but let it be here
    }
}
