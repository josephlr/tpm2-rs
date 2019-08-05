use cc::Build;
use glob::glob;

const CRYPTO_LIB: &'static str = "Ossl";

fn common_build() -> Build {
    let mut build = Build::new();
    build
        .include("ms-tpm-20-ref/TPMCmd/tpm/include")
        .include("ms-tpm-20-ref/TPMCmd/tpm/include/prototypes")
        .include("ms-tpm-20-ref/TPMCmd/Platform/include")
        .include("ms-tpm-20-ref/TPMCmd/Platform/include/prototypes")
        .define("VTPM", "NO")
        .define("SIMULATION", "NO")
        .define("USE_DA_USED", "NO")
        .define("HASH_LIB", CRYPTO_LIB)
        .define("SYM_LIB", CRYPTO_LIB)
        .define("MATH_LIB", CRYPTO_LIB)
        .warnings(true)
        .flag_if_supported("-std=gnu11")
        .flag_if_supported("-Wformat-security")
        .flag_if_supported("-fstack-protector-all");
    build
}

fn main() {
    let platform_files = glob("ms-tpm-20-ref/TPMCmd/Platform/src/*.c")
        .expect("cannot find platform sources")
        .filter_map(Result::ok);
    common_build()
        .files(platform_files)
        .flag_if_supported("-Wno-unused-parameter")
        .compile("platform");

    let tpm_files = glob("ms-tpm-20-ref/TPMCmd/tpm/src/**/*.c")
        .expect("cannot find tpm sources")
        .filter_map(Result::ok);
    common_build()
        .files(tpm_files)
        .flag_if_supported("-Wno-cast-function-type")
        .flag_if_supported("-Wno-implicit-fallthrough")
        .flag_if_supported("-Wno-missing-field-initializers")
        .compile("tpm");
    println!("cargo:rustc-link-lib=crypto");
}
