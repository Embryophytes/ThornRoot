use crate::schema::column::Column;
use crate::schema::relationship::Relationship;

#[derive(Debug, Clone)]
pub struct MigrationPlan {
    pub steps: Vec<MigrationStep>,
}

#[derive(Debug, Clone)]
pub enum MigrationStep {
    CreateTable { name: String },
    DropTable { name: String },
    AddColumn { table: String, column: Column },
    RemoveColumn { table: String, column_name: String },
    AlterColumn { table: String, column: Column },
    AddRelationship { relationship: Relationship },
}
