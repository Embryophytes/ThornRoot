use crate::migration::migration_plan::MigrationPlan;
use crate::schema::Schema;

pub trait DatabaseEngine: Default {
    /// Returns the name of the database engine (e.g., "PostgreSQL", "MySQL").
    fn name() -> &'static str;

    /// Generates a migration plan based on the current and desired schema.
    fn generate_migration_plan(&self, schema: &mut Schema) -> Result<MigrationPlan, String>;
}
