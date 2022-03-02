use std::{thread::sleep, time::Duration};

use tpm::{raw::Tpm, Result};

fn get_tpm() -> Result<Tpm> {
    #[cfg(feature = "test-hardware")]
    return Tpm::get();

    #[cfg(not(feature = "test-hardware"))]
    return tpm_simulator::Simulator::get_tpm();
}

#[test]
fn get_random() -> Result<()> {
    let mut tpm = get_tpm()?;

    let mut output = [0u8; 100];
    let len = tpm.get_random(&mut output)?;
    assert_ne!(len, 100);

    let mut output = [0u8; 10];
    let len = tpm.get_random(&mut output)?;
    assert_eq!(len, 10);

    println!("{:?}", output);
    Ok(())
}

#[test]
fn stir_random() -> Result<()> {
    let mut tpm = get_tpm()?;
    tpm.stir_random(&[0u8; 10])?;
    Ok(())
}

#[test]
fn read_clock() -> Result<()> {
    let mut tpm = get_tpm()?;
    println!("{:?}", tpm.read_clock()?);

    sleep(Duration::from_millis(10));
    println!("{:?}", tpm.read_clock()?);
    Ok(())
}

#[test]
#[cfg(not(feature = "test-hardware"))]
fn read_clock_reset() -> Result<()> {
    let mut driver = tpm_simulator::Simulator::get()?;
    {
        let mut tpm = Tpm::new(&mut driver);
        println!("{:?}", tpm.read_clock()?);

        sleep(Duration::from_millis(10));
        println!("{:?}", tpm.read_clock()?);
    }
    driver.reset()?;
    println!("\nAfter reset:");
    {
        let mut tpm = Tpm::new(&mut driver);
        println!("{:?}", tpm.read_clock()?);

        sleep(Duration::from_millis(10));
        println!("{:?}", tpm.read_clock()?);
    }
    driver.manufacture_reset()?;
    println!("\nAfter manufacturer reset:");
    {
        let mut tpm = Tpm::new(&mut driver);
        println!("{:?}", tpm.read_clock()?);

        sleep(Duration::from_millis(10));
        println!("{:?}", tpm.read_clock()?);
    }
    Ok(())
}
