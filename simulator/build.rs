use cc::Build;
use glob::glob;

fn main() {
    let tpm_files = glob("ms-tpm-20-ref/TPMCmd/tpm/src/**/*.c")
        .expect("cannot find tpm sources")
        .filter_map(Result::ok);

    Build::new()
        .include("ms-tpm-20-ref/TPMCmd/tpm/include")
        .include("ms-tpm-20-ref/TPMCmd/tpm/include/prototypes")
        .include("ms-tpm-20-ref/TPMCmd/Platform/include")
        .include("ms-tpm-20-ref/TPMCmd/Platform/include/prototypes")
        .files(tpm_files)
        .define("DEBUG", "YES")
        .define("SIMULATION", "NO")
        .define("COMPILER_CHECKS", "DEBUG")
        .define("RUNTIME_SIZE_CHECKS", "DEBUG")
        .define("USE_DA_USED", "NO")
        .define("CERTIFYX509_DEBUG", "NO")
        .flag_if_supported("-Wformat-nonliteral")
        .flag_if_supported("-fstack-protector-all")
        .flag_if_supported("-Wno-cast-function-type")
        .flag_if_supported("-Wno-implicit-fallthrough")
        .flag_if_supported("-Wno-missing-field-initializers")
        .flag_if_supported("-Wno-empty-body")
        .flag_if_supported("-Wno-braced-scalar-init")
        .compile("tpm");

    println!("cargo:rustc-link-lib=crypto");
    for path_res in glob("ms-tpm-20-ref/TPMCmd/**/*").unwrap() {
        if let Ok(path) = path_res {
            println!("cargo:rerun-if-changed={}", path.display());
        }
    }
}
