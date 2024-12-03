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

    pub fn get_sql(&self) -> String {
        let mut res = String::default();
        for step in self.steps.iter() {
            res.push_str(step.get_sql_script());
        }
        res
    }
}

#[derive(Debug, Clone)]
pub enum MigrationStep {
    CreateTable {
        name: String,
        sql_script: String,
    },
    DropTable {
        name: String,
        sql_script: String,
    },
    AddColumn {
        table: String,
        column: Column,
        sql_script: String,
    },
    RemoveColumn {
        table: String,
        column_name: String,
        sql_script: String,
    },
    AlterColumn {
        table: String,
        column: Column,
        sql_script: String,
    },
    AddRelationship {
        relationship: Relationship,
        sql_script: String,
    },
}

impl MigrationStep {
    pub fn get_sql_script(&self) -> &str {
        match self {
            MigrationStep::CreateTable { sql_script, .. } => sql_script,
            MigrationStep::DropTable { sql_script, .. } => sql_script,
            MigrationStep::AddColumn { sql_script, .. } => sql_script,
            MigrationStep::RemoveColumn { sql_script, .. } => sql_script,
            MigrationStep::AlterColumn { sql_script, .. } => sql_script,
            MigrationStep::AddRelationship { sql_script, .. } => sql_script,
        }
    }
}
