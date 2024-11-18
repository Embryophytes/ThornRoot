#[derive(Debug, Clone)]
pub struct Relationship {
    pub from_table: String, // not sure about strings but we will see
    pub from_column: String,
    pub to_table: String,
    pub to_column: String,
    pub relationship_type: RelationshipType,
}

#[derive(Debug, Clone)]
pub enum RelationshipType {
    OneToOne,
    OneToMany,
    ManyToMany,
}
