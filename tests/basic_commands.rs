use tpm::{raw::Tpm, Result};

#[cfg(feature = "test-hardware")]
type TestTpm = tpm::OsTpm;
#[cfg(not(feature = "test-hardware"))]
type TestTpm = tpm_simulator::Simulator;

#[test]
fn get_random() -> Result<()> {
    let mut tpm = TestTpm::get()?;
    let mut buf = [0u8; 100];

    let output = tpm.get_random(&mut buf)?;
    assert_ne!(output.len(), 100);

    let output = tpm.get_random(&mut buf[..10])?;
    assert_eq!(output.len(), 10);

    println!("{:?}", output);
    Ok(())
}

#[test]
fn stir_random() -> Result<()> {
    let mut tpm = TestTpm::get()?;
    tpm.stir_random(&[0u8; 10])?;
    Ok(())
}

#[test]
fn read_clock() -> Result<()> {
    use std::thread::sleep;
    use std::time::Duration;
    {
        let mut tpm = TestTpm::get()?;
        println!("{:?}", tpm.read_clock()?);

        sleep(Duration::from_millis(10));
        println!("{:?}", tpm.read_clock()?);

        #[cfg(not(feature = "test-hardware"))]
        tpm.reset()?;
        println!("\nAfter reset:");
        println!("{:?}", tpm.read_clock()?);

        sleep(Duration::from_millis(10));
        println!("{:?}", tpm.read_clock()?);
    }
    {
        let mut tpm = TestTpm::get()?;
        println!("\nAfter manufacturer reset:");
        println!("{:?}", tpm.read_clock()?);

        sleep(Duration::from_millis(10));
        println!("{:?}", tpm.read_clock()?);
    }
    Ok(())
}
