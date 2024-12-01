use super::data_type::DataType;

#[derive(Debug, Clone)]
pub struct Column {
    name: String,
    data_type: DataType,
    primary_key: bool,
    foreign_key: bool,
    nullable: bool,
}

impl Default for Column {
    fn default() -> Self {
        Self {
            name: Default::default(),
            data_type: DataType::Integer,
            primary_key: Default::default(),
            foreign_key: Default::default(),
            nullable: Default::default(),
        }
    }
}

impl Column {
    pub fn new(
        name: &str,
        data_type: DataType,
        primary_key: bool,
        foreign_key: bool,
        nullable: bool,
    ) -> Self {
        Self {
            name: name.to_string(),
            data_type,
            primary_key,
            foreign_key,
            nullable,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_data_type(&self) -> DataType {
        self.data_type.to_owned()
    }

    pub fn is_primary_key(&self) -> bool {
        self.primary_key
    }

    pub fn is_foreign_key(&self) -> bool {
        self.foreign_key
    }

    pub fn is_nullable(&self) -> bool {
        self.nullable
    }

    pub fn get_name_mut(&mut self) -> &mut String {
        &mut self.name
    }

    pub fn get_data_type_mut(&mut self) -> &mut DataType {
        &mut self.data_type
    }

    pub fn is_primary_key_mut(&mut self) -> &mut bool {
        &mut self.primary_key
    }

    pub fn is_foreign_key_mut(&mut self) -> &mut bool {
        &mut self.foreign_key
    }

    pub fn is_nullable_mut(&mut self) -> &mut bool {
        &mut self.nullable
    }
}
