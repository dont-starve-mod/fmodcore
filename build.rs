fn main() {
    println!(r"cargo:rustc-link-search=./");

    #[cfg(target_os="macos")]
    println!(r"cargo:rustc-link-lib=fmodevent");
    #[cfg(target_os="macos")]
    println!(r"cargo:rustc-link-lib=fmodex");

    #[cfg(target_os="windows")]
    println!(r"cargo:rustc-link-lib=fmod_event64");
    #[cfg(target_os="windows")]
    println!(r"cargo:rustc-link-lib=fmodex64");
}