use tpm::{raw::Tpm, Result};

#[cfg(feature = "test-hardware")]
type TestTpm = tpm::OsTpm;
#[cfg(not(feature = "test-hardware"))]
type TestTpm = tpm_simulator::Simulator;

#[test]
fn get_random() -> Result<()> {
    let mut tpm = get_tpm()?;
    let output = tpm.get_random(100)?;
    println!("{:?}", output.bytes());
    Ok(())
}

#[test]
fn stir_random() -> Result<()> {
    let mut tpm = get_tpm()?;
    tpm.stir_random(&[0u8; 10])?;
    tpm.get_random(100)?;
    Ok(())
}

#[test]
fn read_clock() -> Result<()> {
    use std::thread::sleep;
    use std::time::Duration;

    let mut tpm = get_tpm()?;
    println!("{:?}", tpm.read_clock()?);

    sleep(Duration::from_millis(10));
    println!("{:?}", tpm.read_clock()?);

    #[cfg(not(feature = "test_hardware"))]
    tpm.reset()?;
    println!("{:?}", tpm.read_clock()?);

    sleep(Duration::from_millis(10));
    println!("{:?}", tpm.read_clock()?);
    Ok(())
}