use crate::migration::migration_plan::MigrationPlan;
use crate::schema::Schema;

use super::engine::DatabaseEngine;

#[derive(Default)]
pub struct PostgresEngine;

impl DatabaseEngine for PostgresEngine {
    fn name(&self) -> &'static str {
        "PostgreSQL"
    }

    fn generate_migration_plan(&self, _schema: &Schema) -> Result<MigrationPlan, String> {
        Ok(MigrationPlan::default())
    }
}
