use crate::Result;

pub trait CommandData {
    fn marshal_handles(&self, _: &mut &mut [u8]) -> Result<()> {
        Ok(())
    }
    fn marshal_params(&self, buf: &mut &mut [u8]) -> Result<()>;
}

pub trait ResponseData<'a> {
    fn unmarshal_handles(&mut self, _: &mut &[u8]) -> Result<()> {
        Ok(())
    }
    fn unmarshal_params(&mut self, buf: &mut &'a [u8]) -> Result<()>;
}

impl ResponseData<'_> for () {
    fn unmarshal_params(&mut self, _: &mut &[u8]) -> Result<()> {
        Ok(())
    }
}

mod structs;
pub use structs::*;
