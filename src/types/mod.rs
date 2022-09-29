use crate::{error::UnmarshalError, polyfill::ToArr, MarshalFixed, UnmarshalAny, Unmarshal};

mod auth;
pub use auth::*;
mod buffer;
pub use buffer::*;

pub mod tpm;
pub mod tpma;
pub mod tpml;
pub mod tpms;
pub mod tpmt;
pub mod tpmu;

pub type Handle = u32;

#[derive(Debug)]
pub(crate) struct CommandHeader {
    pub tag: tpm::ST,
    pub size: u32,
    pub code: tpm::CC,
}

impl MarshalFixed for CommandHeader {
    const SIZE: usize = 10;
    type ARRAY = [u8; Self::SIZE];

    fn marshal_fixed(&self, arr: &mut Self::ARRAY) {
        self.tag.marshal_fixed(arr[0..2].to_arr());
        self.size.marshal_fixed(arr[2..6].to_arr());
        self.code.marshal_fixed(arr[6..10].to_arr());
    }
}

#[derive(Debug, Default)]
pub(crate) struct ResponseHeader {
    pub tag: tpm::ST,
    pub size: u32,
    pub code: tpm::RC,
}
impl Unmarshal<'_> for ResponseHeader {
    fn unmarshal(&mut self, buf: &mut &[u8]) -> Result<(), UnmarshalError> {
        self.tag.unmarshal(buf)?;
        self.size.unmarshal(buf)?;
        self.code.unmarshal(buf)?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn header_size() {
        assert_eq!(CommandHeader::SIZE, 10);
        // assert_eq!(ResponseHeader::SIZE, 10);
    }
}
