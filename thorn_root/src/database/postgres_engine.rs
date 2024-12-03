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


#[cfg(test)]
mod tests {
    use crate::schema::column::Column;
    use crate::schema::data_type;

    use super::*;
    #[test]
    fn test_2_tables_with_column_in_each() {
        let mut schema = Schema::new();
        schema.add_table("users").unwrap();
        schema.add_table("orders").unwrap();

        let users_table = schema.get_table_mut("users").unwrap();
        users_table
            .add_column(Column::new(
                "id",
                data_type::DataType::Integer,
                true,
                false,
                false,
            ))
            .unwrap();

        let orders_table = schema.get_table_mut("orders").unwrap();
        orders_table
            .add_column(Column::new(
                "user_id",
                data_type::DataType::Integer,
                false,
                true,
                false,
            ))
            .unwrap();

        let postgres_engine = PostgresEngine::default();
        let res = postgres_engine.generate_migration_plan(&schema);
        assert!(res.is_ok());
    }

}

