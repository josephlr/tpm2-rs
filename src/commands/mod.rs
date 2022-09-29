use crate::error::{MarshalError, UnmarshalError};

pub trait CommandData {
    fn marshal_handles(&self, _: &mut &mut [u8]) -> Result<(), MarshalError> {
        Ok(())
    }
    fn marshal_params(&self, _: &mut &mut [u8]) -> Result<(), MarshalError> {
        Ok(())
    }
}

pub trait ResponseData<'a> {
    fn unmarshal_handles(&mut self, _: &mut &[u8]) -> Result<(), UnmarshalError> {
        Ok(())
    }
    fn unmarshal_params(&mut self, _: &mut &'a [u8]) -> Result<(), UnmarshalError> {
        Ok(())
    }
}

impl ResponseData<'_> for () {}

mod structs;
pub use structs::*;
