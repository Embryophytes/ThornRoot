use ion_rs;
use ion_rs::IonReader;
use ion_rs::IonWriter;

use super::decoder::Decoder;
use super::encoder::Encoder;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ColumnDTO {
    name: String,
    data_type: String, // Representing the data type as a string for serialization
    is_primary_key: bool,
    is_nullable: bool,
    api_code: String,
}

impl ColumnDTO {
    pub fn new(
        name: String,
        data_type: String,
        is_primary_key: bool,
        is_nullable: bool,
        api_code: String,
    ) -> Self {
        ColumnDTO {
            name,
            data_type,
            is_primary_key,
            is_nullable,
            api_code,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_data_type(&self) -> &str {
        &self.data_type
    }

    pub fn is_primary_key(&self) -> bool {
        self.is_primary_key
    }

    pub fn is_nullable(&self) -> bool {
        self.is_nullable
    }

    pub fn get_api_code(&self) -> &str {
        &self.api_code
    }
}

impl Encoder for ColumnDTO {
    fn encode(&self) -> Vec<u8> {
        let buffer: Vec<u8> = Vec::new();

        let binary_writer_builder = ion_rs::BinaryWriterBuilder::new();
        let mut writer = binary_writer_builder.build(buffer.clone()).unwrap();

        writer
            .step_in(ion_rs::IonType::Struct)
            .expect("Error while creating an ion struct");

        writer.set_field_name("name");
        writer.write_string(&self.name).unwrap();

        writer.set_field_name("data_type");
        writer.write_string(&self.data_type).unwrap();

        writer.set_field_name("is_primary_key");
        writer.write_bool(self.is_primary_key).unwrap();

        writer.set_field_name("is_nullable");
        writer.write_bool(self.is_nullable).unwrap();

        writer.set_field_name("api_code");
        writer.write_string(&self.api_code).unwrap();

        writer.step_out().unwrap();
        writer.flush().unwrap();

        writer.output().as_slice().to_owned()
    }
}

impl Decoder for ColumnDTO {
    fn decode(data: &[u8]) -> Self {
        let mut reader = ion_rs::ReaderBuilder::new().build(data).unwrap();
        reader.next().unwrap();
        reader.step_in().unwrap();

        reader.next().unwrap();
        let name = reader.read_string().unwrap().text().to_string();

        reader.next().unwrap();
        let data_type = reader.read_string().unwrap().text().to_string();

        reader.next().unwrap();
        let is_primary_key = reader.read_bool().unwrap();

        reader.next().unwrap();
        let is_nullable = reader.read_bool().unwrap();
        reader.next().unwrap();
        let api_code = reader.read_string().unwrap().text().to_string();

        ColumnDTO::new(name, data_type, is_primary_key, is_nullable, api_code)
    }
}

#[cfg(test)]
mod tests {
    use super::super::decoder::Decoder;
    use super::super::encoder::Encoder;

    use super::ColumnDTO;

    #[test]
    fn test_column_dto_encoding_and_decoding() {
        const NAME: &str = "column_name";
        const DATA_TYPE: &str = "VARCHAR";
        const IS_PRIMARY_KEY: bool = true;
        const IS_NULLABLE: bool = false;
        const API_CODE: &str = "API_CODE";

        let column = ColumnDTO::new(
            NAME.to_string(),
            DATA_TYPE.to_string(),
            IS_PRIMARY_KEY,
            IS_NULLABLE,
            API_CODE.to_string(),
        );

        let encoded = column.encode();
        let decoded = ColumnDTO::decode(&encoded);

        assert_eq!(column, decoded);
    }

    #[test]
    fn test_column_dto_fields() {
        const NAME: &str = "column_name";
        const DATA_TYPE: &str = "VARCHAR";
        const IS_PRIMARY_KEY: bool = true;
        const IS_NULLABLE: bool = false;
        const API_CODE: &str = "API_CODE";

        let column = ColumnDTO::new(
            NAME.to_string(),
            DATA_TYPE.to_string(),
            IS_PRIMARY_KEY,
            IS_NULLABLE,
            API_CODE.to_string(),
        );

        assert_eq!(column.get_name(), NAME);
        assert_eq!(column.get_data_type(), DATA_TYPE);
        assert_eq!(column.is_primary_key(), IS_PRIMARY_KEY);
        assert_eq!(column.is_nullable(), IS_NULLABLE);
        assert_eq!(column.get_api_code(), API_CODE);
    }
}
