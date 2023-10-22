pub use socketcan;

/// Really simple error cases
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("General error, dunno what")]
    CannotDecodeOk,
    #[error("Invalid slice length")]
    InvalidSlicingLength,
    #[error("Invalid bytes conversion")]
    InvalidBytesConversion,
}

/// Define a means to consume an 8 byte CAN frame and give us a type
pub trait CanDecode: Sized {
    fn from_socketcan(frame: [u8; 8]) -> Result<Self, Error>;
}

pub mod helper {
    use crate::Error;

    /// Advances a counting token and returns the next bit of the slice
    /// Note that we don't use [] as its panic-able
    pub fn advance_token<'a, T: Sized>(
        offset: &'a mut usize,
        frame: &'a [u8],
    ) -> Result<&'a [u8], Error> {
        let extract_bytes = std::mem::size_of::<T>();
        advance_token_by(offset, frame, extract_bytes)
    }

    pub fn advance_token_by<'a>(
        offset: &'a mut usize,
        frame: &'a [u8],
        extract_bytes: usize,
    ) -> Result<&'a [u8], Error> {
        let next = *offset + extract_bytes;
        let slice = frame
            .get(*offset..next)
            .ok_or_else(|| Error::InvalidSlicingLength)?;
        *offset = next;
        Ok(slice)
    }

    /// Extracts bytes for an offset
    /// Note that we don't use [] as its panic-able
    pub fn extract_offset<'a, T: Sized>(offset: usize, frame: &'a [u8]) -> Result<&'a [u8], Error> {
        let extract_bytes = std::mem::size_of::<T>();
        extract_offset_by(offset, frame, extract_bytes)
    }

    pub fn extract_offset_by<'a>(
        offset: usize,
        frame: &'a [u8],
        extract_bytes: usize,
    ) -> Result<&'a [u8], Error> {
        let next = offset + extract_bytes;
        let slice = frame
            .get(offset..next)
            .ok_or_else(|| Error::InvalidSlicingLength)?;
        Ok(slice)
    }
}
