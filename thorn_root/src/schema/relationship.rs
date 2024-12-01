#[derive(Debug, Clone)]
pub struct Relationship {
    from_table: String,
    from_column: String,
    to_table: String,
    to_column: String,
    relationship_type: RelationshipType,
    code: String,
}

impl Relationship {
    pub fn new(
        from_table: &str,
        from_column: &str,
        to_table: &str,
        to_column: &str,
        relationship_type: RelationshipType,
    ) -> Self {
        Self {
            from_table: from_table.to_string(),
            from_column: from_column.to_string(),
            to_table: to_table.to_string(),
            to_column: to_column.to_string(),
            relationship_type,
            code: format!("{}_{}_{}_{}", from_table, from_column, to_table, to_column),
        }
    }

    /// Getter for `from_table`
    pub fn get_from_table(&self) -> &str {
        &self.from_table
    }

    /// Getter for `from_column`
    pub fn get_from_column(&self) -> &str {
        &self.from_column
    }

    /// Getter for `to_table`
    pub fn get_to_table(&self) -> &str {
        &self.to_table
    }

    /// Getter for `to_column`
    pub fn get_to_column(&self) -> &str {
        &self.to_column
    }

    /// Getter for `relationship_type`
    pub fn get_relationship_type(&self) -> &RelationshipType {
        &self.relationship_type
    }

    /// Getter for `code`
    pub fn get_code(&self) -> &str {
        &self.code
    }

    pub fn set_from_table(&mut self, from_table: &str) {
        self.from_table = from_table.to_string();
        self.code = format!(
            "{}_{}_{}_{}",
            &self.from_table, &self.from_column, &self.to_table, &self.to_column
        );
    }

    pub fn set_from_column(&mut self, from_column: &str) {
        self.from_column = from_column.to_string();
        self.code = format!(
            "{}_{}_{}_{}",
            &self.from_table, &self.from_column, &self.to_table, &self.to_column
        );
    }

    pub fn set_to_table(&mut self, to_table: &str) {
        self.to_table = to_table.to_string();
        self.code = format!(
            "{}_{}_{}_{}",
            &self.from_table, &self.from_column, &self.to_table, &self.to_column
        );
    }

    pub fn set_to_column(&mut self, to_column: &str) {
        self.to_column = to_column.to_string();
        self.code = format!(
            "{}_{}_{}_{}",
            &self.from_table, &self.from_column, &self.to_table, &self.to_column
        );
    }

    pub fn set_relationship_type(&mut self, relationship_type: RelationshipType) {
        self.relationship_type = relationship_type;
    }
}

#[derive(Debug, Clone)]
pub enum RelationshipType {
    OneToOne,
    OneToMany,
}
