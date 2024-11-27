use ion_rs;
use ion_rs::element::reader::ElementReader;
use ion_rs::{IonReader, IonWriter};

use super::decoder::Decoder;
use super::encoder::Encoder;
use super::table_dto::TableDTO;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SchemaDTO {
    tables: Vec<TableDTO>,
    version: u8,
}

impl SchemaDTO {
    pub fn new(tables: Vec<TableDTO>, version: u8) -> Self {
        SchemaDTO { tables, version }
    }

    pub fn get_tables(&self) -> &Vec<TableDTO> {
        &self.tables
    }

    pub fn get_version(&self) -> u8 {
        self.version
    }
}

impl Encoder for SchemaDTO {
    fn encode(&self) -> Vec<u8> {
        let buffer: Vec<u8> = Vec::new();

        let binary_writer_builder = ion_rs::BinaryWriterBuilder::new();
        let mut writer = binary_writer_builder.build(buffer.clone()).unwrap();

        writer
            .step_in(ion_rs::IonType::Struct)
            .expect("Error while creating an ion struct");

        writer.set_field_name("tables");
        writer.step_in(ion_rs::IonType::List).unwrap();
        for table in &self.tables {
            writer.write_blob(&table.encode()).unwrap();
        }
        writer.step_out().unwrap();

        writer.set_field_name("version");
        writer.write_i64(self.version.into()).unwrap(); // Writing u8 as i64

        writer.step_out().unwrap();
        writer.flush().unwrap();

        writer.output().as_slice().to_owned()
    }
}

impl Decoder for SchemaDTO {
    fn decode(data: &[u8]) -> Self {
        let mut reader = ion_rs::ReaderBuilder::new().build(data).unwrap();
        reader.next().unwrap();
        reader.step_in().unwrap();

        reader.next().unwrap();
        reader.step_in().unwrap();
        let elements = reader.read_all_elements().unwrap();
        let mut tables = Vec::with_capacity(elements.len());
        for el in elements.into_iter() {
            tables.push(TableDTO::decode(el.as_blob().unwrap()));
        }
        reader.step_out().unwrap();

        reader.next().unwrap();
        let version = reader.read_i64().unwrap() as u8; // Reading as i64, converting to u8

        SchemaDTO::new(tables, version)
    }
}

#[cfg(test)]
mod tests {
    use super::super::column_dto::ColumnDTO;
    use super::super::decoder::Decoder;
    use super::super::encoder::Encoder;
    use super::super::relationship_dto::RelationshipDTO;
    use super::super::table_dto::TableDTO;
    use super::SchemaDTO;

    #[test]
    fn test_schema_dto_encoding_and_decoding() {
        const VERSION: u8 = 1;

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
            "users".to_string(),
            vec![column],
            vec![relationship],
            false,
            "TABLE_API_CODE".to_string(),
        );

        let schema = SchemaDTO::new(vec![table], VERSION);

        let encoded = schema.encode();
        let decoded = SchemaDTO::decode(&encoded);

        assert_eq!(schema, decoded);
    }

    #[test]
    fn test_schema_dto_fields() {
        const VERSION: u8 = 1;

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
            "users".to_string(),
            vec![column.clone()],
            vec![relationship.clone()],
            false,
            "TABLE_API_CODE".to_string(),
        );

        let schema = SchemaDTO::new(vec![table.clone()], VERSION);

        assert_eq!(schema.get_tables(), &vec![table]);
        assert_eq!(schema.get_version(), VERSION);
    }
}
