use crate::migration::migration_plan::MigrationPlan;
use crate::schema::table::Table;

use super::engine::DatabaseEngine;

#[derive(Default)]
pub struct PostgresEngine;

impl DatabaseEngine for PostgresEngine {
    fn name(&self) -> &'static str {
        "PostgreSQL"
    }

    fn apply_migration(&self, _migration: &MigrationPlan) -> Result<(), String> {
        Ok(())
    }

    fn generate_migration_plan(
        &self,
        _tables: &[Table],
    ) -> Result<MigrationPlan, String> {
        // need to generate the migration plan based on some existing schema
        // I guess I do need here to compare the existing tables with DTO structures
        Ok(MigrationPlan { steps: vec![] })
    }
}
