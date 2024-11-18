#[derive(Debug, Clone)]
pub enum CoreError {
    DatabaseError(String),
    SchemaValidationError(String),
    MigrationError(String),
}
