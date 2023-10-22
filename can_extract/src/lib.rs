pub use socketcan;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("data store disconnected")]
    CannotDecodeOk,
    #[error("Invalid slice length")]
    InvalidSlicingLength,
    #[error("Invalid bytes conversion")]
    InvalidBytesConversion,
}

pub trait CanDecode: Sized {
    fn from_socketcan(frame: [u8; 8]) -> Result<Self, Error>;
}

pub mod helper {
    use crate::Error;

    pub fn advance_token<'a, T: Sized>(
        offset: &'a mut usize,
        frame: &'a [u8],
    ) -> Result<&'a [u8], Error> {
        let val = std::mem::size_of::<T>();
        let next = *offset + val;
        let slice = frame
            .get(*offset..next)
            .ok_or_else(|| Error::InvalidSlicingLength)?;
        *offset = next;
        Ok(slice)
    }
}
