use cc::Build;
use glob::glob;

fn main() {
    if std::fs::read_dir("ms-tpm-20-ref").unwrap().count() == 0 {
        eprintln!("error: submodule \"ms-tpm-20-ref\" is not initialized");
        eprintln!("error: try running: git submodule update --init");
        std::process::abort();
    }

    let tpm_files = glob("ms-tpm-20-ref/TPMCmd/tpm/src/**/*.c").unwrap();
    let google_files = glob("ms-tpm-20-ref/Samples/Google/*.c").unwrap();

    Build::new()
        .include("ms-tpm-20-ref/TPMCmd/tpm/include")
        .include("ms-tpm-20-ref/TPMCmd/tpm/include/prototypes")
        .include("ms-tpm-20-ref/Samples/Google")
        .files(tpm_files.filter_map(Result::ok))
        .files(google_files.filter_map(Result::ok))
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

    for dir in &[
        "ms-tpm-20-ref/TPMCmd/tpm/**",
        "ms-tpm-20-ref/Samples/Google",
    ] {
        for path_res in glob(&format!("{}/*.[hc]", dir)).unwrap() {
            if let Ok(path) = path_res {
                println!("cargo:rerun-if-changed={}", path.display());
            }
        }
    }
}
