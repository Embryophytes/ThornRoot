use super::data_type::DataType;

#[derive(Debug, Clone)]
pub struct Column {
    pub name: String,
    pub data_type: DataType,
    pub is_primary_key: bool,
    pub is_nullable: bool,
    api_code: String
}

impl Column {
    pub fn get_api_code(&self) -> &str {
        self.api_code.as_str()
    }
}
