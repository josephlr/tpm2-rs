//! TODO: Explain the substructure here

use crate::{error::UnmarshalError, polyfill::ToArr, MarshalFixed, Unmarshal};

mod auth;
pub use auth::*;

pub mod tpm;
pub mod tpm2b;
pub mod tpma;
pub mod tpmi;
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

pub trait GetAlg {
    fn alg(&self) -> tpm::Alg;
}

impl<T: GetAlg> GetAlg for Option<T> {
    fn alg(&self) -> tpm::Alg {
        match self {
            Some(t) => t.alg(),
            None => tpm::Alg::Null,
        }
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
