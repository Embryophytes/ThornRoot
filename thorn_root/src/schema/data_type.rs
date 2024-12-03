use strum_macros::EnumIter;

use crate::database::{engine::DatabaseEngine, postgres_engine::PostgresEngine};

#[derive(Debug, Clone, PartialEq, Eq, EnumIter)]
pub enum DataType {
    Integer,
    Float,
    String,
    Boolean,
    Date,
}
impl DataType {
    pub fn to_db_type(&self, db: &str) -> Option<&'static str> {
        let postgres_value = PostgresEngine::name();
        match db {
            postgres_value => match self {
                DataType::Integer => Some("INTEGER"),
                DataType::Float => Some("DOUBLE PRECISION"),
                DataType::String => Some("TEXT"),
                DataType::Boolean => Some("BOOLEAN"),
                DataType::Date => Some("DATE"),
            },
            "oracle" => match self {
                DataType::Integer => Some("NUMBER"),
                DataType::Float => Some("FLOAT"),
                DataType::String => Some("VARCHAR2"),
                DataType::Boolean => Some("NUMBER(1)"),
                DataType::Date => Some("DATE"),
            },
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use strum::IntoEnumIterator;

    use super::*;
    #[test]
    fn test_iterate() {
        for (index, enum_value) in DataType::iter().enumerate() {
            assert_eq!(index, enum_value as usize);
        }
    }
}
