use super::{FixedSize, Marshal, Result, Unmarshal};

trait Attribute: Default + Copy {
    type Raw: Default + FixedSize + Marshal + for<'a> Unmarshal<'a>;

    fn from_raw(raw: Self::Raw) -> Self;
    fn to_raw(self) -> Self::Raw;
    fn get_bit(&self, n: usize) -> bool;
    fn set_bit(&mut self, n: usize, b: bool);
}

impl<A: Attribute> Marshal for A {
    fn marshal(&self, buf: &mut &mut [u8]) -> Result<()> {
        self.to_raw().marshal(buf)
    }
}

impl<A: Attribute> Unmarshal<'_> for A {
    fn unmarshal(&mut self, buf: &mut &[u8]) -> Result<()> {
        *self = Self::from_raw(A::Raw::unmarshal_val(buf)?);
        Ok(())
    }
}

impl<A: Attribute> FixedSize for A {
    const SIZE: usize = A::Raw::SIZE;
}

#[derive(PartialEq, Eq, Clone, Copy, Default, Debug)]
pub struct Session {
    reserved: u8,
    pub continue_session: bool,
    pub audit_exclusive: bool,
    pub audit_reset: bool,
    pub decrypt: bool,
    pub encrypt: bool,
    pub audit: bool,
}

impl Session {
    pub const fn empty() -> Self {
        Self {
            reserved: 0,
            continue_session: false,
            audit_exclusive: false,
            audit_reset: false,
            decrypt: false,
            encrypt: false,
            audit: false,
        }
    }
}

#[derive(Clone, Copy, Default)]
pub struct Memory {
    reserved: u32,
    pub shared_ram: bool,
    pub shared_nv: bool,
    pub object_copied_to_ram: bool,
}

impl Attribute for Memory {
    type Raw = u32;

    fn from_raw(raw: Self::Raw) -> Self {
        Self {
            shared_ram: raw & (1 << 0) != 0,
            shared_nv: raw & (1 << 1) != 0,
            object_copied_to_ram: raw & (1 << 2) != 0,
            reserved: raw & !(0b111),
        }
    }

    fn to_raw(self) -> Self::Raw {
        let mut raw = self.reserved & !(0b111);
        if self.shared_ram {
            raw |= 1 << 0;
        }
        if self.shared_nv {
            raw |= 1 << 1;
        }
        if self.object_copied_to_ram {
            raw |= 1 << 2;
        }
        raw
    }

    fn get_bit(&self, n: usize) -> bool {
        self.to_raw() & (1 << n) != 0
    }

    fn set_bit(&mut self, n: usize, b: bool) {
        match (n, b) {
            (0, _) => self.shared_ram = b,
            (1, _) => self.shared_nv = b,
            (2, _) => self.object_copied_to_ram = b,
            (_, true) => self.reserved |= 1 << n,
            (_, false) => self.reserved &= !(1 << n),
        }
    }
}

impl Attribute for Session {
    type Raw = u8;

    fn from_raw(raw: Self::Raw) -> Self {
        Self {
            continue_session: raw & (1 << 0) != 0,
            audit_exclusive: raw & (1 << 1) != 0,
            audit_reset: raw & (1 << 2) != 0,
            decrypt: raw & (1 << 5) != 0,
            encrypt: raw & (1 << 6) != 0,
            audit: raw & (1 << 7) != 0,
            reserved: raw & !(0b11100111),
        }
    }

    fn to_raw(self) -> Self::Raw {
        let mut raw = self.reserved & !(0b11100111);
        if self.continue_session {
            raw |= 1 << 0;
        }
        if self.audit_exclusive {
            raw |= 1 << 1;
        }
        if self.audit_reset {
            raw |= 1 << 2;
        }
        if self.decrypt {
            raw |= 1 << 5;
        }
        if self.encrypt {
            raw |= 1 << 6;
        }
        if self.audit {
            raw |= 1 << 7;
        }
        raw
    }

    fn get_bit(&self, n: usize) -> bool {
        self.to_raw() & (1 << n) != 0
    }

    fn set_bit(&mut self, n: usize, b: bool) {
        match (n, b) {
            (0, _) => self.continue_session = b,
            (1, _) => self.audit_exclusive = b,
            (2, _) => self.audit_reset = b,
            (5, _) => self.decrypt = b,
            (6, _) => self.encrypt = b,
            (7, _) => self.audit = b,
            (_, true) => self.reserved |= 1 << n,
            (_, false) => self.reserved &= !(1 << n),
        }
    }
}
