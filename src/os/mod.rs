cfg_if::cfg_if! {
    if #[cfg(not(feature = "std"))] {
        // Freestanding targets don't have builtin Tpm bindings
    } else if #[cfg(target_os = "linux")] {
        mod linux;
        pub use linux::OsTpm;
    } else if #[cfg(windows)] {
        mod windows;
        pub use windows::OsTpm;
    } else {
        // Other targets are not yet supported
    }
}
