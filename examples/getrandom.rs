use tpm2::{
    commands::{Command, GetRandom},
    os::get_default_tpm,
};

const NUM_BYTES: u16 = 100;

fn main() {
    let mut tpm = get_default_tpm().expect("Unable to open TPM");

    let cmd = GetRandom {
        bytes_requested: NUM_BYTES,
    };
    println!("Requesting {} random bytes from the TPM", NUM_BYTES);

    let rsp = cmd.run(&mut tpm).expect("TPM2_GetRandom failed");
    println!("Got {} random bytes:", rsp.random_bytes.len());
    for chunk in rsp.random_bytes.chunks(8) {
        println!("\t{:02X?}", chunk)
    }
}
