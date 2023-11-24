fn main() {
    println!(r"cargo:rustc-link-search=./");

    #[cfg(macos)]
    println!(r"cargo:rustc-link-lib=fmodevent");
    #[cfg(macos)]
    println!(r"cargo:rustc-link-lib=fmodex");

    #[cfg(windows)]
    println!(r"cargo:rustc-link-lib=fmod_event64");
    #[cfg(windows)]
    println!(r"cargo:rustc-link-lib=fmodex64");
}