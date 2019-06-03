fn main() {
    println!("cargo:rustc-link-search=native=/opt/devkitpro/libogc/lib/wii");
    println!("cargo:rustc-link-lib=static=ogc");
}
