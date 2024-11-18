use crate::migration::migration_plan::MigrationPlan;

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
        _current_schema: &str,
        _desired_schema: &str,
    ) -> Result<MigrationPlan, String> {
        Ok(MigrationPlan { steps: vec![] })
    }
}
