use std::env::args;

use tpm2::{commands::ReadPublic, os::default_tpm, types::Handle, TpmRun};

fn parse_handle(s: &str) -> Handle {
    if let Some(hex) = s.strip_prefix("0x") {
        Handle::from_str_radix(hex, 16)
    } else {
        s.parse()
    }
    .expect("invalid handle number")
}

fn main() {
    let mut tpm = default_tpm().expect("Unable to open TPM");

    let object_handle = parse_handle(&args().nth(1).expect("no handle provided"));

    let rsp = tpm
        .run(ReadPublic { object_handle })
        .expect("ReadPublic failed");

    println!("Public: {:#?}", rsp.public);
    println!("Name: {:x?}", rsp.name);
    println!("Qualified Name: {:x?}", rsp.qualified_name);
}
