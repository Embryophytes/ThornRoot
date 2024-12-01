use relationship::Relationship;
use table::Table;

pub mod column;
pub mod data_type;
pub mod relationship;
pub mod table;

/// Represents the schema of a database, which includes tables and relationships.
pub struct Schema {
    /// The collection of tables in the schema.
    tables: Vec<Table>,
    /// The collection of relationships between tables in the schema.
    relations: Vec<Relationship>,
}

impl Default for Schema {
    /// Provides a default implementation for `Schema` by creating a new, empty schema.
    fn default() -> Self {
        Self::new()
    }
}

impl Schema {
    /// Creates a new, empty `Schema`.
    ///
    /// # Returns
    /// - A new instance of `Schema` with no tables or relationships.
    pub fn new() -> Self {
        Self {
            tables: Vec::new(),
            relations: Vec::new(),
        }
    }

    /// Gets relationships for a given table.
    ///
    /// # Arguments
    /// - `table_name`: The name of the table whose relationships are to be retrieved.
    ///
    /// # Returns
    /// - A `Result` containing a vector of references to `Relationship` if found, or an error message.
    pub fn get_relationships_for_table(&self, table_name: &str) -> Result<Vec<&Relationship>, String> {
        let _ = self.get_table(table_name)?;
        let relationship = self
            .relations
            .iter()
            .filter(|r| r.get_from_table() == table_name || r.get_to_table() == table_name)
            .collect::<Vec<&Relationship>>();

        Ok(relationship)
    }

    /// Gets the tables involved in a given relationship.
    ///
    /// # Arguments
    /// - `relation_code`: The unique code of the relationship.
    ///
    /// # Returns
    /// - A `Result` containing a vector of references to `Table` if found, or an error message.
    pub fn get_tables_for_relationships(&self, relation_code: &str) -> Result<Vec<&Table>, String> {
        let relation = self.get_relationship_with_code(relation_code)?;
        let tables = self
            .tables
            .iter()
            .filter(|t| t.get_name() == relation.get_from_table() || t.get_name() == relation.get_to_table())
            .collect::<Vec<&Table>>();

        Ok(tables)
    }

    /// Retrieves a specific relationship by its unique code.
    ///
    /// # Arguments
    /// - `relation_code`: The code of the relationship to retrieve.
    ///
    /// # Returns
    /// - A `Result` containing a reference to the `Relationship` if found, or an error message.
    pub fn get_relationship_with_code(&self, relation_code: &str) -> Result<&Relationship, String> {
        match self.relations.iter().find(|r| r.get_code() == relation_code) {
            Some(relationship) => Ok(relationship),
            None => Err(format!("Relationship with code {} doesn't exist", relation_code)),
        }
    }

    /// Retrieves a specific relationship by its unique code.
    ///
    /// # Arguments
    /// - `relation_code`: The code of the relationship to retrieve.
    ///
    /// # Returns
    /// - A `Result` containing a mutable reference to the `Relationship` if found, or an error message.
    pub fn get_relationship_with_code_mut(&mut self, relation_code: &str) -> Result<&mut Relationship, String> {
        match self.relations.iter_mut().find(|r| r.get_code() == relation_code) {
            Some(relationship) => Ok(relationship),
            None => Err(format!("Relationship with code {} doesn't exist", relation_code)),
        }
    }

    /// Retrieves all tables in the schema.
    ///
    /// # Returns
    /// - A slice of all tables in the schema.
    pub fn get_tables(&self) -> &[Table] {
        &self.tables
    }

    /// Retrieves a mutable reference to all tables in the schema.
    ///
    /// # Returns
    /// - A mutable reference to the vector of tables.
    pub fn get_tables_mut(&mut self) -> &mut Vec<Table> {
        &mut self.tables
    }

    /// Retrieves a table by its name.
    ///
    /// # Arguments
    /// - `table_name`: The name of the table to retrieve.
    ///
    /// # Returns
    /// - A `Result` containing a reference to the `Table` if found, or an error message.
    pub fn get_table(&self, table_name: &str) -> Result<&Table, String> {
        match self.tables.iter().find(|t| t.get_name() == table_name) {
            Some(table) => Ok(table),
            None => Err(format!("Table with name {} doesn't exist", table_name)),
        }
    }

    /// Retrieves a mutable reference to a table by its name.
    ///
    /// # Arguments
    /// - `table_name`: The name of the table to retrieve.
    ///
    /// # Returns
    /// - A `Result` containing a mutable reference to the `Table` if found, or an error message.
    pub fn get_table_mut(&mut self, table_name: &str) -> Result<&mut Table, String> {
        match self.tables.iter_mut().find(|t| t.get_name() == table_name) {
            Some(table) => Ok(table),
            None => Err(format!("Table with name {} doesn't exist", table_name)),
        }
    }

    /// Retrieves all relationships in the schema.
    ///
    /// # Returns
    /// - A slice of all relationships in the schema.
    pub fn get_relationships(&self) -> &[Relationship] {
        &self.relations
    }

    /// Retrieves a mutable reference to all relationships in the schema.
    ///
    /// # Returns
    /// - A mutable reference to the vector of relationships
    pub fn get_relationships_mut(&mut self) -> &mut Vec<Relationship> {
        &mut self.relations
    }

    /// Adds a new table to the schema.
    ///
    /// # Arguments
    /// - `table_name`: The name of the table to add.
    ///
    /// # Returns
    /// - A `Result` containing a reference to the newly added table, or an error message if the table already exists.
    pub fn add_table(&mut self, table_name: &str) -> Result<&Table, String> {
        let index = self.tables.iter().position(|t| t.get_name() == table_name);
        match index {
            Some(_) => {
                Err(format!("Table with name {} already exists", table_name))
            },
            None => {
                self.tables.push(Table::new(table_name));
                Ok(self.tables.last().unwrap())
            },
        } 
    }

    /// Removes a table from the schema.
    ///
    /// # Arguments
    /// - `table_name`: The name of the table to remove.
    ///
    /// # Returns
    /// - A `Result` containing the removed table, or an error message if the table does not exist.
    pub fn remove_table(&mut self, table_name: &str) -> Result<Table, String> {
        let index = self
            .tables
            .iter()
            .position(|t| t.get_name() == table_name)
            .ok_or(format!("Table with name {} doesn't exist", table_name))?;
        Ok(self.tables.swap_remove(index))
    }

    /// Adds a new relationship to the schema.
    ///
    /// # Arguments
    /// - `relationship`: The `Relationship` to add.
    ///
    /// # Returns
    /// - A `Result` containing a reference to the newly added relationship, or an error message if the relationship already exists.
    pub fn add_relation(
        &mut self,
        relationship: Relationship,        
    ) -> Result<&Relationship, String> {
        let from_table = self
            .get_table(relationship.get_from_table());

        if from_table.is_err() {
            return Err("`From table` must be defined".to_string());
        }

        let from_table = from_table.unwrap();

        let to_table = self
            .get_table(relationship.get_to_table());

        if to_table.is_err() {
            return Err("`To table` must be defined".to_string());
        }

        let to_table = to_table.unwrap();

        let from_column = from_table.get_column(relationship.get_from_column());

        if from_column.is_err() {
            return Err("`From column` must be defined".to_string());
        }

        let to_column = to_table.get_column(relationship.get_to_column());

        if to_column.is_err() {
            return Err("`To column` must be defined".to_string());
        }

        if self.get_relationship_with_code(relationship.get_code()).is_ok() {
            return Err("Such a relationship is already created".to_string());
        }

        self.relations.push(relationship);
        Ok(self.relations.last().unwrap())
    }

    /// Updates an existing relationship in the schema.
    ///
    /// # Arguments
    /// - `relationship`: The updated `Relationship` object.
    ///
    /// # Returns
    /// - A `Result` containing a reference to the updated relationship, or an error message if the relationship does not exist.
    pub fn update_relation(
        &mut self,
        relationship: &Relationship,
    ) -> Result<&Relationship, String> {
        self
            .relations
            .iter()
            .position(|r| r.get_code() == relationship.get_code())
            .ok_or("Such a realtion doesn't exist".to_string())?;

        if relationship.get_from_table() == relationship.get_to_table() && relationship.get_from_column() == relationship.get_to_column() {
            return Err("You can't create a relations to the same column of the same table".to_string());
        }

        let from_table = self.get_table(relationship.get_from_table())?;
        let to_table = self.get_table(relationship.get_to_table())?;

        let from_column = from_table.get_column(relationship.get_from_column())?;
        let to_column = to_table.get_column(relationship.get_to_column())?;

        if from_column.get_data_type() != to_column.get_data_type() {
            return Err("You can't create a relations to columns of different data types".to_string());
        }

        self.delete_relation(relationship.get_code())?;

        self.add_relation(Relationship::new(
            relationship.get_from_table(),
            relationship.get_from_column(),
            relationship.get_to_table(),
            relationship.get_to_column(),
            relationship.get_relationship_type(),
        ))
    }

    /// Deletes a relationship from the schema.
    ///
    /// # Arguments
    /// - `relation_code`: The code of the relationship to delete.
    ///
    /// # Returns
    /// - A `Result` containing the deleted relationship, or an error message if the relationship does not exist.
    pub fn delete_relation(
        &mut self,
        relation_code: &str,
    ) -> Result<Relationship, String> {
        let index = self
            .relations
            .iter()
            .position(|r| r.get_code() == relation_code)
            .ok_or("Such a realtion doesn't exist".to_string())?;
        
        Ok(self.relations.swap_remove(index))
    }
}

#[cfg(test)]
mod tests {
    use column::Column;

    use super::*;

    #[test]
    fn test_create_empty_schema() {
        let schema = Schema::new();
        assert!(schema.get_tables().is_empty());
        assert!(schema.get_relationships().is_empty());
    }

    #[test]
    fn test_add_table() {
        let mut schema = Schema::new();
        let table_name = "users";
        assert!(schema.add_table(table_name).is_ok());
        assert_eq!(schema.get_tables().len(), 1);
        assert_eq!(schema.get_tables()[0].get_name(), table_name);
    }

    #[test]
    fn test_add_duplicate_table() {
        let mut schema = Schema::new();
        let table_name = "users";
        schema.add_table(table_name).unwrap();
        let result = schema.add_table(table_name);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), format!("Table with name {} already exists", table_name));
    }

    #[test]
    fn test_remove_table() {
        let mut schema = Schema::new();
        let table_name = "users";
        schema.add_table(table_name).unwrap();
        let removed_table = schema.remove_table(table_name).unwrap();
        assert_eq!(removed_table.get_name(), table_name);
        assert!(schema.get_tables().is_empty());
    }

    #[test]
    fn test_remove_nonexistent_table() {
        let mut schema = Schema::new();
        let result = schema.remove_table("nonexistent");
        assert!(result.is_err());
    }

    #[test]
    fn test_add_relationship() {
        let mut schema = Schema::new();
        schema.add_table("users").unwrap();
        schema.add_table("orders").unwrap();

        let users_table = schema.get_table_mut("users").unwrap();
        users_table.add_column(Column::new(
            "id",
            data_type::DataType::Integer,
            true,
            false,
            false,
        )).unwrap();

        let orders_table = schema.get_table_mut("orders").unwrap();
        orders_table.add_column(Column::new(
            "user_id",
            data_type::DataType::Integer,
            false,
            true,
            false,
        )).unwrap();

        let relationship = Relationship::new(
            "users",
            "id",
            "orders",
            "user_id",
            relationship::RelationshipType::OneToMany,
        );
        assert!(schema.add_relation(relationship).is_ok());
        assert_eq!(schema.get_relationships().len(), 1);
    }

    #[test]
    fn test_add_relationship_err() {
        let mut schema = Schema::new();
        schema.add_table("users").unwrap();
        schema.add_table("orders").unwrap();

        let relationship = Relationship::new(
            "users",
            "id",
            "orders",
            "user_id",
            relationship::RelationshipType::OneToMany,
        );
        assert!(schema.add_relation(relationship).is_err());
        assert_eq!(schema.get_relationships().len(), 0);
    }

    #[test]
    fn test_add_duplicate_relationship() {
        let mut schema = Schema::new();
        schema.add_table("users").unwrap();
        schema.add_table("orders").unwrap();

        let users_table = schema.get_table_mut("users").unwrap();
        users_table.add_column(Column::new(
            "id",
            data_type::DataType::Integer,
            true,
            false,
            false,
        )).unwrap();

        let orders_table = schema.get_table_mut("orders").unwrap();
        orders_table.add_column(Column::new(
            "user_id",
            data_type::DataType::Integer,
            false,
            true,
            false,
        )).unwrap();

        let relationship = Relationship::new(
            "users",
            "id",
            "orders",
            "user_id",
            relationship::RelationshipType::OneToMany,
        );
        schema.add_relation(relationship.clone()).unwrap();
        let result = schema.add_relation(relationship);
        assert!(result.is_err());
        assert_eq!(schema.get_relationships().len(), 1);
    }

    #[test]
    fn test_get_relationship_with_code() {
        let mut schema = Schema::new();
        schema.add_table("users").unwrap();
        schema.add_table("orders").unwrap();

        let users_table = schema.get_table_mut("users").unwrap();
        users_table.add_column(Column::new(
            "id",
            data_type::DataType::Integer,
            true,
            false,
            false,
        )).unwrap();

        let orders_table = schema.get_table_mut("orders").unwrap();
        orders_table.add_column(Column::new(
            "user_id",
            data_type::DataType::Integer,
            false,
            true,
            false,
        )).unwrap();

        let relationship = Relationship::new(
            "users",
            "id",
            "orders",
            "user_id",
            relationship::RelationshipType::OneToMany,
        );
        let code = relationship.get_code().to_owned();
        schema.add_relation(relationship).unwrap();
        let retrieved_relationship = schema.get_relationship_with_code(&code).unwrap();
        assert_eq!(retrieved_relationship.get_code(), code);
    }

    #[test]
    fn test_get_relationship_for_nonexistent_code() {
        let schema = Schema::new();
        let result = schema.get_relationship_with_code("nonexistent");
        assert!(result.is_err());
    }

    #[test]
    fn test_get_relationships_for_table() {
        let mut schema = Schema::new();
        schema.add_table("users").unwrap();
        schema.add_table("orders").unwrap();
        schema.add_table("products").unwrap();

        let users_table = schema.get_table_mut("users").unwrap();
        users_table.add_column(Column::new(
            "id",
            data_type::DataType::Integer,
            true,
            false,
            false,
        )).unwrap();

        let orders_table = schema.get_table_mut("orders").unwrap();
        orders_table.add_column(Column::new(
            "user_id",
            data_type::DataType::Integer,
            false,
            true,
            false,
        )).unwrap();
        orders_table.add_column(Column::new(
            "product_id",
            data_type::DataType::Integer,
            false,
            true,
            false,
        )).unwrap();

        let products_table = schema.get_table_mut("products").unwrap();
        products_table.add_column(Column::new(
            "id",
            data_type::DataType::Integer,
            true,
            false,
            false,
        )).unwrap();

        let rel1 = Relationship::new("users", "id", "orders", "user_id", relationship::RelationshipType::OneToOne);
        let rel2 = Relationship::new("orders", "product_id", "products", "id", relationship::RelationshipType::OneToMany);
        schema.add_relation(rel1).unwrap();
        schema.add_relation(rel2).unwrap();
        let relationships = schema.get_relationships_for_table("orders").unwrap();
        assert_eq!(relationships.len(), 2);
    }

    #[test]
    fn test_update_relation() {
        let mut schema = Schema::new();

        // Add necessary tables to the schema
        schema.add_table("users").unwrap();
        schema.add_table("orders").unwrap();

        let users_table = schema.get_table_mut("users").unwrap();
        users_table.add_column(Column::new(
            "id",
            data_type::DataType::Integer,
            true,
            false,
            false,
        )).unwrap();

        let orders_table = schema.get_table_mut("orders").unwrap();
        orders_table.add_column(Column::new(
            "user_id",
            data_type::DataType::Integer,
            false,
            true,
            false,
        )).unwrap();

        // Add initial relationship
        let initial_relationship = Relationship::new(
            "users",
            "id",
            "orders",
            "user_id",
            relationship::RelationshipType::OneToMany,
        );
        let initial_code = initial_relationship.get_code().to_string();
        schema.add_relation(initial_relationship).unwrap();

        // Retrieve the relationship and modify it
        let mut updated_relationship = schema.get_relationship_with_code(&initial_code).unwrap().clone();
        updated_relationship.set_relationship_type(relationship::RelationshipType::OneToOne);

        // Update the relationship in the schema
        let result = schema.update_relation(&updated_relationship);
        assert!(result.is_ok());
        let updated_relation_code = result.unwrap().get_code().to_owned();

        // Verify the relationship is updated correctly
        let updated = schema.get_relationship_with_code(&updated_relation_code).unwrap();
        assert_eq!(*updated, updated_relationship);
    }


    #[test]
    fn test_delete_relationship() {
        let mut schema = Schema::new();
        schema.add_table("users").unwrap();
        schema.add_table("orders").unwrap();

        let users_table = schema.get_table_mut("users").unwrap();
        users_table.add_column(Column::new(
            "id",
            data_type::DataType::Integer,
            true,
            false,
            false,
        )).unwrap();

        let orders_table = schema.get_table_mut("orders").unwrap();
        orders_table.add_column(Column::new(
            "user_id",
            data_type::DataType::Integer,
            false,
            true,
            false,
        )).unwrap();

        let relationship = Relationship::new(
            "users",
            "id",
            "orders",
            "user_id",
            relationship::RelationshipType::OneToOne,
        );
        let code = relationship.get_code().to_owned();
        schema.add_relation(relationship).unwrap();
        let deleted_relationship = schema.delete_relation(&code).unwrap();
        assert_eq!(deleted_relationship.get_code(), code);
        assert!(schema.get_relationship_with_code(&code).is_err());
    }
}
