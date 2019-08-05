pub use tpm_core::data;
pub use tpm_core::{Error, Result, Tpm};

#[cfg(feature = "tpm_os")]
pub use tpm_os::OsTpm;

#[cfg(test)]
mod tests {
    use super::*;
    use once_cell::sync::OnceCell;

    use core::ops::DerefMut;
    use std::sync::Mutex;

    #[cfg(feature = "test_hardware")]
    type TestTpm = tpm_os::OsTpm;
    #[cfg(not(feature = "test_hardware"))]
    type TestTpm = tpm_simulator::Simulator;

    fn get_tpm() -> Result<impl DerefMut<Target = TestTpm>> {
        static TPM: OnceCell<Mutex<TestTpm>> = OnceCell::new();
        let tpm = TPM.get_or_try_init(|| TestTpm::get().map(Mutex::new))?;
        Ok(tpm.lock().unwrap())
    }

    #[test]
    fn get_random() -> Result<()> {
        let mut tpm = get_tpm()?;
        let output = tpm.get_random(100)?;
        println!("{:?}", output.bytes());
        Ok(())
    }

    #[test]
    fn read_clock() -> Result<()> {
        let mut tpm = get_tpm()?;
        let info = tpm.read_clock()?;
        println!("{:?}", info);
        Ok(())
    }
}
