pub use tpm_core::data;
pub use tpm_core::{Error, Result, Tpm};

#[cfg(feature = "tpm_os")]
pub use tpm_os::OsTpm;

#[cfg(test)]
mod tests {
    use core::ops::DerefMut;
    use std::sync::Mutex;

    use super::*;
    use once_cell::sync::Lazy;

    fn get_tpm() -> impl DerefMut<Target = OsTpm> {
        static TPM: Lazy<Mutex<OsTpm>> =
            Lazy::new(|| Mutex::new(OsTpm::new("/dev/tpm0").expect("Unable to get TPM")));
        TPM.lock().unwrap()
    }

    #[test]
    fn get_random() -> Result<()> {
        let mut tpm = get_tpm();
        let _: &mut dyn Tpm = tpm.deref_mut();
        let output = tpm.get_random(100)?;
        println!("{:?}", output.bytes());
        Ok(())
    }

    #[test]
    fn read_clock() -> Result<()> {
        let mut tpm = get_tpm();
        let info = tpm.read_clock()?;
        println!("{:?}", info);
        Ok(())
    }
}
