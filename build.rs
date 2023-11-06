fn main() {
    println!(r"cargo:rustc-link-search=./");
    println!(r"cargo:rustc-link-lib=fmodevent");
    println!(r"cargo:rustc-link-lib=fmodex");
}