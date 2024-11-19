use std::sync::Arc;

use crate::database::engine::DatabaseEngine;

use super::column::Column;
use super::relationship::Relationship;
use super::table::Table;


pub struct SchemaBuilder<T>
where
    T: DatabaseEngine {
    tables: Vec<Table>,
    database_engine: Arc<T>,
}

impl<T> Default for SchemaBuilder<T>
where
    T: DatabaseEngine
 {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> SchemaBuilder<T>
where
    T: DatabaseEngine {
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
    pub fn add_column(&mut self, table_name: &str, column: Column, _save_with_error: bool) -> Result<(), String> {
        match self.tables.iter_mut().find(|t| t.name == table_name) {
            Some(table) => {
                table.add_column(column);
                Ok(())
            }
            None => Err(format!("Table '{}' not found", table_name)),
        }
    }

    /// Updates a column in an existing table.
    pub fn update_column(&mut self, _updated_column: Column, _save_with_error: bool) -> Result<(), String> {
        todo!();
        // Perform a validation if any relations to this entity has been added
        // probably 
    }

    /// Deletes a column from an existing table.
    pub fn delete_column(&mut self, _updated_column: Column, _save_with_error: bool) -> Result<(), String> {
        todo!();
        // Perform a validation if any relations to this entity has been added
        // probably 
    }

    /// Add a relation to the schema.
    pub fn add_relation(&mut self, _relationship: Relationship, _save_with_error: bool) -> Result<(), String> {
        todo!();        
    }

    /// Updates a relation in the schema.
    pub fn update_relation(&mut self, _updated_relationship: Relationship, _save_with_error: bool) -> Result<(), String> {
        todo!();        
    }

    /// Deletes a relation from the schema.
    pub fn delete_relation(&mut self, _updated_relationship: Relationship, _save_with_error: bool) -> Result<(), String> {
        todo!();        
    }

    /// Builds the schema and returns the result.
    pub fn build(self) -> Vec<Table> {
        self.tables
    }
    
    pub fn database_engine(&self) -> &T {
        &self.database_engine
    }
}
