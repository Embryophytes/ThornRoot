use crate::schema::column::Column;
use crate::schema::relationship::Relationship;

#[derive(Debug, Default, Clone)]
pub struct MigrationPlan {
    steps: Vec<MigrationStep>,
}

impl MigrationPlan {
    pub fn new(steps: Vec<MigrationStep>) -> Self {
        Self { steps }
    }

    pub fn get_steps(&self) -> &[MigrationStep] {
        &self.steps
    }
}

impl ToString for MigrationPlan {
    fn to_string(&self) -> String {
        todo!()
    }
}

#[derive(Debug, Clone)]
pub enum MigrationStep {
    CreateTable { name: String, sql_script: String },
    DropTable { name: String, sql_script: String },
    AddColumn { table: String, column: Column, sql_script: String },
    RemoveColumn { table: String, column_name: String, sql_script: String },
    AlterColumn { table: String, column: Column, sql_script: String },
    AddRelationship { relationship: Relationship, sql_script: String },
}
