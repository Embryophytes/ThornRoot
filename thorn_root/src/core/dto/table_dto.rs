use ion_rs;
use ion_rs::element::reader::ElementReader;
use ion_rs::{IonReader, IonWriter};

use super::column_dto::ColumnDTO;
use super::relationship_dto::RelationshipDTO;
use super::decoder::Decoder;
use super::encoder::Encoder;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TableDTO {
    name: String,
    columns: Vec<ColumnDTO>,
    relationships: Vec<RelationshipDTO>,
    saved_with_error: bool,
    api_code: String,
}

impl TableDTO {
    pub fn new(
        name: String,
        columns: Vec<ColumnDTO>,
        relationships: Vec<RelationshipDTO>,
        saved_with_error: bool,
        api_code: String,
    ) -> Self {
        TableDTO {
            name,
            columns,
            relationships,
            saved_with_error,
            api_code,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_columns(&self) -> &Vec<ColumnDTO> {
        &self.columns
    }

    pub fn get_relationships(&self) -> &Vec<RelationshipDTO> {
        &self.relationships
    }

    pub fn is_saved_with_error(&self) -> bool {
        self.saved_with_error
    }

    pub fn get_api_code(&self) -> &str {
        &self.api_code
    }
}

impl Encoder for TableDTO {
    fn encode(&self) -> Vec<u8> {
        let buffer: Vec<u8> = Vec::new();

        let binary_writer_builder = ion_rs::BinaryWriterBuilder::new();
        let mut writer = binary_writer_builder.build(buffer.clone()).unwrap();

        writer.step_in(ion_rs::IonType::Struct).expect("Error while creating an ion struct");

        writer.set_field_name("name");
        writer.write_string(&self.name).unwrap();

        writer.set_field_name("columns");
        writer.step_in(ion_rs::IonType::List).unwrap();
        for column in &self.columns {
            writer.write_blob(&column.encode()).unwrap();
        }
        writer.step_out().unwrap();

        writer.set_field_name("relationships");
        writer.step_in(ion_rs::IonType::List).unwrap();
        for relationship in &self.relationships {
            writer.write_blob(&relationship.encode()).unwrap();
        }
        writer.step_out().unwrap();

        writer.set_field_name("saved_with_error");
        writer.write_bool(self.saved_with_error).unwrap();

        writer.set_field_name("api_code");
        writer.write_string(&self.api_code).unwrap();

        writer.step_out().unwrap();
        writer.flush().unwrap();

        writer.output().as_slice().to_owned()
    }
}

impl Decoder for TableDTO {
    fn decode(data: &[u8]) -> Self {
        let mut reader = ion_rs::ReaderBuilder::new().build(data).unwrap();
        reader.next().unwrap();
        reader.step_in().unwrap();

        reader.next().unwrap();
        let name = reader.read_string().unwrap().text().to_string();

        reader.next().unwrap();
        reader.step_in().unwrap();
        let elements = reader.read_all_elements().unwrap();
        let mut columns = Vec::with_capacity(elements.len());
        for el in elements.into_iter() {
            columns.push(ColumnDTO::decode(el.as_blob().unwrap()));
        }
        reader.step_out().unwrap();

        reader.next().unwrap();
        reader.step_in().unwrap();
        let elements = reader.read_all_elements().unwrap();
        let mut relationships = Vec::with_capacity(elements.len());
        for el in elements.into_iter() {
            relationships.push(RelationshipDTO::decode(el.as_blob().unwrap()));
        }
        reader.step_out().unwrap();

        reader.next().unwrap();
        let saved_with_error = reader.read_bool().unwrap();

        reader.next().unwrap();
        let api_code = reader.read_string().unwrap().text().to_string();

        TableDTO::new(name, columns, relationships, saved_with_error, api_code)
    }
}

#[cfg(test)]
mod tests {
    use super::super::column_dto::ColumnDTO;
    use super::super::relationship_dto::RelationshipDTO;
    use super::super::decoder::Decoder;
    use super::super::encoder::Encoder;
    use super::TableDTO;

    #[test]
    fn test_table_dto_encoding_and_decoding() {
        const TABLE_NAME: &str = "users";
        const API_CODE: &str = "API_CODE";

        let column = ColumnDTO::new(
            "id".to_string(),
            "INTEGER".to_string(),
            true,
            false,
            "COLUMN_API_CODE".to_string(),
        );
        let relationship = RelationshipDTO::new(
            "users".to_string(),
            "id".to_string(),
            "orders".to_string(),
            "user_id".to_string(),
            1, // OneToMany
        );

        let table = TableDTO::new(
            TABLE_NAME.to_string(),
            vec![column],
            vec![relationship],
            false,
            API_CODE.to_string(),
        );

        let encoded = table.encode();
        let decoded = TableDTO::decode(&encoded);

        assert_eq!(table, decoded);
    }

    #[test]
    fn test_table_dto_fields() {
        const TABLE_NAME: &str = "users";
        const API_CODE: &str = "API_CODE";

        let column = ColumnDTO::new(
            "id".to_string(),
            "INTEGER".to_string(),
            true,
            false,
            "COLUMN_API_CODE".to_string(),
        );
        let relationship = RelationshipDTO::new(
            "users".to_string(),
            "id".to_string(),
            "orders".to_string(),
            "user_id".to_string(),
            1, // OneToMany
        );

        let table = TableDTO::new(
            TABLE_NAME.to_string(),
            vec![column.clone()],
            vec![relationship.clone()],
            false,
            API_CODE.to_string(),
        );

        assert_eq!(table.get_name(), TABLE_NAME);
        assert_eq!(table.get_columns(), &vec![column]);
        assert_eq!(table.get_relationships(), &vec![relationship]);
        assert!(!table.is_saved_with_error());
        assert_eq!(table.get_api_code(), API_CODE);
    }
}
