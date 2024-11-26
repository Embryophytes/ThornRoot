#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DataType {
    Integer,
    Float,
    String,
    Boolean,
    Date,
    Custom(String),
}
// need to write a converter to string for the DataType
