use tpm2::{commands::ReadClock, os::default_tpm, TpmRaw};

fn main() {
    let mut tpm = default_tpm().expect("Unable to open TPM");

    println!("Reading the current Clock");
    let rsp = tpm.run(&ReadClock {}).expect("TPM2_ReadClock failed");
    println!("{:?}", rsp.current_time);
}
