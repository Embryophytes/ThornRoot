use crate::migration::migration_plan::{MigrationPlan, MigrationStep};
use crate::schema::Schema;

use super::engine::DatabaseEngine;

#[derive(Default)]
pub struct PostgresEngine;

impl DatabaseEngine for PostgresEngine {
    fn name() -> &'static str {
        "PostgreSQL"
    }

    fn generate_migration_plan(&self, schema: &mut Schema) -> Result<MigrationPlan, String> {
        // Create the tables
        // if a table doesn't contain a primary key then one will be created as COL_ID

        // create table template
        /*
        CREATE TABLE example_table (
            column1 data_type1 NOT NULL,
            column2 data_type2, -- NULL by default
            column3 data_type3 PRIMARY KEY
        );

        // if col is PK it can't be unique or not null because it is already PK
        // other fields can be one

        // add foreign keys - a so called references
        ALTER TABLE example_table
        ADD CONSTRAINT column2_fk FOREIGN KEY (column2) REFERENCES another_table (another_column);

        // You are done!
         */
        let mut migration_steps = Vec::<MigrationStep>::default();

        let tables = schema.get_tables();
        for table in tables {
            // let mut migration_step = MigrationStep::CreateTable
            let mut create_table_tmp = format!(r#"
                CREATE TABLE {} (
                    {{}}
                );
            "#, table.get_name());
            for (index, column) in table.get_columns().iter().enumerate() {
                let table_to_place = if index == table.get_columns().len() - 1 {
                    format!("{} {}",
                        column.get_name(),
                        column.get_data_type().to_db_type(PostgresEngine::name()).unwrap()
                    )
                    // can update here other constraints
                } else {
                    format!("{} {} {{}}",
                        column.get_name(),
                        column.get_data_type().to_db_type(PostgresEngine::name()).unwrap()
                    )
                    // can update here other constraints
                };
                create_table_tmp = create_table_tmp.replace(
                    "{}",
                    &table_to_place,
                );
            }
            migration_steps.push(MigrationStep::CreateTable { 
                name: table.get_name().to_string(),
                sql_script: create_table_tmp,
            });
        }

        let relations = schema.get_relationships();
        for relation in relations {
            let create_relation_tmp = format!(
                r#"
                    ALTER TABLE {} -- table name
                    ADD CONSTRAINT {}_fk FOREIGN KEY ({}) REFERENCES {} ({});
                "#,
                relation.get_to_table(),
                relation.get_to_column(),
                relation.get_to_column(),
                relation.get_from_table(),
                relation.get_from_column(),
            );
            migration_steps.push(MigrationStep::AddRelationship {
                relationship: relation.clone(),
                sql_script: create_relation_tmp,
            });
        }
        Ok(MigrationPlan::new(migration_steps))
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
        let res = postgres_engine.generate_migration_plan(&mut schema);
        assert!(res.is_ok());
        let plan = res.unwrap();
        let sql = plan.get_sql();
        println!("{}", sql);
    }

}

