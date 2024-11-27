pub trait Decoder {
    fn decode(data: &[u8]) -> Self
    where
        Self: Sized;
}
