use super::relationship::Relationship;
use super::table::Table;

pub struct Schema {
    tables: Vec<Table>,
    relations: Vec<Relationship>,
}

impl Default for Schema {
    fn default() -> Self {
        Self::new()
    }
}

impl Schema {
    /// Creates a new, empty `SchemaBuilder`.
    pub fn new() -> Self {
        Self {
            tables: Vec::new(),
            relations: Vec::new(),
        }
    }

    /// Gets relations for a given table
    pub fn get_relationships_for_table(&self, _table_name: &str) -> Vec<&Relationship> {
        todo!()
    }

    pub fn get_tables_for_relationships(&self, _relation_code: &str) -> Result<Vec<Table>, String> {
        // I hope it will make sense
        todo!()
    }

    pub fn get_tables(&self) -> &[Table] {
        &self.tables
    }

    pub fn get_tables_mut(&mut self) -> &mut Vec<Table> {
        &mut self.tables
    }

    pub fn get_table(&self, _api_code: &str) -> Result<&Table, String> {
        todo!()
    }

    pub fn get_table_mut(&mut self, _api_code: &str) -> Result<&mut Table, String> {
        todo!()
    }

    pub fn get_relationships(&self) -> &[Relationship] {
        &self.relations
    }

    pub fn get_relationships_mut(&mut self) -> &mut Vec<Relationship> {
        &mut self.relations
    }

    /// Adds a new table to the schema.
    pub fn add_table(&mut self, table_name: &str) -> &mut Table {
        let table = Table::new(table_name);
        self.tables.push(table);
        self.tables.last_mut().unwrap()
    }

    /// Removes the table
    pub fn remove_table(&mut self, _table_name: &str) -> Result<(), String> {
        todo!()
    }

    /// Add a relation to the schema.
    pub fn add_relation(
        &mut self,
        _relationship: Relationship,
        _save_with_error: bool,
    ) -> Result<(), String> {
        todo!();
    }

    /// Updates a relation in the schema.
    pub fn update_relation(
        &mut self,
        _updated_relationship: Relationship,
        _save_with_error: bool,
    ) -> Result<(), String> {
        todo!();
    }

    /// Deletes a relation from the schema.
    pub fn delete_relation(
        &mut self,
        _updated_relationship: Relationship,
        _save_with_error: bool,
    ) -> Result<(), String> {
        todo!();
    }
}
