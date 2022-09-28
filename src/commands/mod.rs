use crate::error::{MarshalError, UnmarshalError};

pub trait CommandData {
    fn marshal_handles(&self, _: &mut &mut [u8]) -> Result<(), MarshalError> {
        Ok(())
    }
    fn marshal_params(&self, buf: &mut &mut [u8]) -> Result<(), MarshalError>;
}

pub trait ResponseData<'a> {
    fn unmarshal_handles(&mut self, _: &mut &[u8]) -> Result<(), UnmarshalError> {
        Ok(())
    }
    fn unmarshal_params(&mut self, buf: &mut &'a [u8]) -> Result<(), UnmarshalError>;
}

impl ResponseData<'_> for () {
    fn unmarshal_params(&mut self, _: &mut &[u8]) -> Result<(), UnmarshalError> {
        Ok(())
    }
}

mod structs;
pub use structs::*;
