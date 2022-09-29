use tpm2::{commands::GetRandom, os::get_default_tpm, Tpm};

const NUM_BYTES: u16 = 100;

fn main() {
    let mut tpm = get_default_tpm().expect("Unable to open TPM");

    run_example(&mut tpm);
    ext_example(&mut tpm);
}

fn run_example(tpm: &mut dyn Tpm) {
    use tpm2::Run;

    println!("Requesting {} random bytes with one command", NUM_BYTES);
    let cmd = GetRandom {
        bytes_requested: NUM_BYTES,
    };
    let rsp = tpm.run(&cmd).expect("TPM2_GetRandom failed");
    println!("Got {} random bytes:", rsp.random_bytes.len());
    print_bytes(rsp.random_bytes);
}

fn ext_example(tpm: &mut dyn Tpm) {
    use tpm2::TpmExt;

    println!(
        "Requesting {} random bytes with multiple commands",
        NUM_BYTES
    );
    let mut buf = [0u8; NUM_BYTES as usize];
    tpm.getrandom(&mut buf).expect("getrandom failed");
    print_bytes(&buf);
}

fn print_bytes(b: &[u8]) {
    for chunk in b.chunks(8) {
        println!("\t{:02X?}", chunk)
    }
}
