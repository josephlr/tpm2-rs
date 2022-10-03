use tpm2::{
    commands::PcrRead,
    os::default_tpm,
    types::{tpm, tpms},
    TpmRun,
};

fn main() {
    let mut tpm = default_tpm().expect("Unable to open TPM");

    // TODO: Query the available banks and read _all_ the PCRs.

    let sel = [
        tpms::PcrSelection {
            hash: tpm::Alg::Sha1,
            select: [true; tpms::NUM_PCRS],
        },
        tpms::PcrSelection {
            hash: tpm::Alg::Sha256,
            select: [true; tpms::NUM_PCRS],
        },
    ];

    let cmd = PcrRead {
        pcr_selection: (&sel).into(),
    };
    println!("Reading from {:?} PCR banks", cmd.pcr_selection.len());
    let rsp = tpm.run(cmd).expect("Unable to read PCRS");

    for sel in rsp.pcr_selection {
        print!("{:?} selection:", sel.hash);
        for (i, &bit) in sel.select.iter().enumerate() {
            if bit {
                print!(" {}", i);
            }
        }
        println!();
    }

    println!("{} digests", rsp.pcr_values.len());
    for digest in rsp.pcr_values {
        print!("\t0x");
        for &b in digest {
            print!("{:02X}", b);
        }
        println!()
    }
}
