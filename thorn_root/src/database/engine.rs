use crate::migration::migration_plan::MigrationPlan;
use crate::schema::table::Table;

pub trait DatabaseEngine: Default {
    /// Returns the name of the database engine (e.g., "PostgreSQL", "MySQL").
    fn name(&self) -> &'static str;

    /// Applies a migration plan to the database.
    fn apply_migration(&self, migration: &MigrationPlan) -> Result<(), String>;

    /// Generates a migration plan based on the current and desired schema.
    fn generate_migration_plan(
        &self,
        tables: &[Table],
    ) -> Result<MigrationPlan, String>;
}
