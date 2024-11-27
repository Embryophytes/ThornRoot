use ion_rs;
use ion_rs::{IonReader, IonWriter};

use super::decoder::Decoder;
use super::encoder::Encoder;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RelationshipDTO {
    from_table: String,
    from_column: String,
    to_table: String,
    to_column: String,
    relationship_type: u8, // Using u8 to represent relationship type
}

impl RelationshipDTO {
    pub fn new(
        from_table: String,
        from_column: String,
        to_table: String,
        to_column: String,
        relationship_type: u8,
    ) -> Self {
        RelationshipDTO {
            from_table,
            from_column,
            to_table,
            to_column,
            relationship_type,
        }
    }

    pub fn get_from_table(&self) -> &str {
        &self.from_table
    }

    pub fn get_from_column(&self) -> &str {
        &self.from_column
    }

    pub fn get_to_table(&self) -> &str {
        &self.to_table
    }

    pub fn get_to_column(&self) -> &str {
        &self.to_column
    }

    pub fn get_relationship_type(&self) -> u8 {
        self.relationship_type
    }
}

impl Encoder for RelationshipDTO {
    fn encode(&self) -> Vec<u8> {
        let buffer: Vec<u8> = Vec::new();

        let binary_writer_builder = ion_rs::BinaryWriterBuilder::new();
        let mut writer = binary_writer_builder.build(buffer.clone()).unwrap();

        writer
            .step_in(ion_rs::IonType::Struct)
            .expect("Error while creating an ion struct");

        writer.set_field_name("from_table");
        writer.write_string(&self.from_table).unwrap();

        writer.set_field_name("from_column");
        writer.write_string(&self.from_column).unwrap();

        writer.set_field_name("to_table");
        writer.write_string(&self.to_table).unwrap();

        writer.set_field_name("to_column");
        writer.write_string(&self.to_column).unwrap();

        writer.set_field_name("relationship_type");
        writer.write_i64(self.relationship_type.into()).unwrap(); // Writing u8 as i64

        writer.step_out().unwrap();
        writer.flush().unwrap();

        writer.output().as_slice().to_owned()
    }
}

impl Decoder for RelationshipDTO {
    fn decode(data: &[u8]) -> Self {
        let mut reader = ion_rs::ReaderBuilder::new().build(data).unwrap();
        reader.next().unwrap();
        reader.step_in().unwrap();

        reader.next().unwrap();
        let from_table = reader.read_string().unwrap().text().to_string();

        reader.next().unwrap();
        let from_column = reader.read_string().unwrap().text().to_string();

        reader.next().unwrap();
        let to_table = reader.read_string().unwrap().text().to_string();

        reader.next().unwrap();
        let to_column = reader.read_string().unwrap().text().to_string();

        reader.next().unwrap();
        let relationship_type = reader.read_i64().unwrap() as u8; // Reading as i64, converting to u8

        RelationshipDTO::new(
            from_table,
            from_column,
            to_table,
            to_column,
            relationship_type,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::super::decoder::Decoder;
    use super::super::encoder::Encoder;

    use super::RelationshipDTO;

    #[test]
    fn test_relationship_dto_encoding_and_decoding() {
        const FROM_TABLE: &str = "users";
        const FROM_COLUMN: &str = "id";
        const TO_TABLE: &str = "orders";
        const TO_COLUMN: &str = "user_id";
        const RELATIONSHIP_TYPE: u8 = 1; // Assuming 0 = OneToOne, 1 = OneToMany

        let relationship = RelationshipDTO::new(
            FROM_TABLE.to_string(),
            FROM_COLUMN.to_string(),
            TO_TABLE.to_string(),
            TO_COLUMN.to_string(),
            RELATIONSHIP_TYPE,
        );

        let encoded = relationship.encode();
        let decoded = RelationshipDTO::decode(&encoded);

        assert_eq!(relationship, decoded);
    }

    #[test]
    fn test_relationship_dto_fields() {
        const FROM_TABLE: &str = "users";
        const FROM_COLUMN: &str = "id";
        const TO_TABLE: &str = "orders";
        const TO_COLUMN: &str = "user_id";
        const RELATIONSHIP_TYPE: u8 = 0; // OneToOne

        let relationship = RelationshipDTO::new(
            FROM_TABLE.to_string(),
            FROM_COLUMN.to_string(),
            TO_TABLE.to_string(),
            TO_COLUMN.to_string(),
            RELATIONSHIP_TYPE,
        );

        assert_eq!(relationship.get_from_table(), FROM_TABLE);
        assert_eq!(relationship.get_from_column(), FROM_COLUMN);
        assert_eq!(relationship.get_to_table(), TO_TABLE);
        assert_eq!(relationship.get_to_column(), TO_COLUMN);
        assert_eq!(relationship.get_relationship_type(), RELATIONSHIP_TYPE);
    }
}
