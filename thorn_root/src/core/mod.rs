pub mod dto;

pub enum ApiCodeType {
    Table,
    Column,
}

impl ApiCodeType {
    pub fn prefix(&self) -> &'static str {
        match self {
            ApiCodeType::Table => "TBL_",
            ApiCodeType::Column => "COL_",
        }
    }
}
pub fn transform_to_api_code(input: &str, code_type: ApiCodeType) -> String {
    let trimmed = input.trim();
    let replaced = trimmed.replace(|c: char| c.is_whitespace(), "_");
    let uppercased = replaced.to_uppercase();
    format!("{}{}", code_type.prefix(), uppercased)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transform_to_api_code_table() {
        let input = "example table name";
        let result = transform_to_api_code(input, ApiCodeType::Table);
        assert_eq!(result, "TBL_EXAMPLE_TABLE_NAME");
    }

    #[test]
    fn test_transform_to_api_code_column() {
        let input = "example column name";
        let result = transform_to_api_code(input, ApiCodeType::Column);
        assert_eq!(result, "COL_EXAMPLE_COLUMN_NAME");
    }

    #[test]
    fn test_transform_with_leading_trailing_whitespace() {
        let input = "  example name  ";
        let result = transform_to_api_code(input, ApiCodeType::Table);
        assert_eq!(result, "TBL_EXAMPLE_NAME");
    }

    #[test]
    fn test_transform_with_internal_whitespace() {
        let input = "example   name";
        let result = transform_to_api_code(input, ApiCodeType::Table);
        assert_eq!(result, "TBL_EXAMPLE___NAME");
    }

    #[test]
    fn test_transform_already_uppercase() {
        let input = "EXAMPLE NAME";
        let result = transform_to_api_code(input, ApiCodeType::Table);
        assert_eq!(result, "TBL_EXAMPLE_NAME");
    }

    #[test]
    fn test_transform_empty_string() {
        let input = "";
        let result = transform_to_api_code(input, ApiCodeType::Table);
        assert_eq!(result, "TBL_");
    }
}
