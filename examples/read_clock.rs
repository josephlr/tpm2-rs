use tpm2::{commands::ReadClock, os::get_default_tpm, Command};

fn main() {
    let mut tpm = get_default_tpm().expect("Unable to open TPM");

    println!("Reading the current Clock");
    let rsp = ReadClock{}.run(&mut tpm).expect("TPM2_ReadClock failed");
    println!("{:?}", rsp.current_time);
}