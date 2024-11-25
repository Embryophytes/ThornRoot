#[derive(Debug, Clone, Eq)]
pub struct Relationship {
    // api codes are used here
    pub from_table: String, // not sure about strings but we will see
    pub from_column: String,
    pub to_table: String,
    pub to_column: String,
    pub relationship_type: RelationshipType,
    // relation by itself doesn't have api_code
    // during building the migration we will just look for the same realation

    // If any - do nothing
    // If some from the previous migration haven't been found - delete
    // If new - create a new one
}


impl PartialEq for Relationship {
    fn eq(&self, other: &Self) -> bool {
        self.from_table == other.from_table
            && self.from_column == other.from_column
            && self.to_table == other.to_table
            && self.to_column == other.to_column
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RelationshipType {
    OneToOne,
    OneToMany,
}
