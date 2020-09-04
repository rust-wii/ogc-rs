fn main() {
    let dkp_path = std::env::var("DEVKITPRO").unwrap();

    println!(
        "cargo:rustc-link-search=native={}/devkitPPC/powerpc-eabi/lib",
        dkp_path
    );
    println!("cargo:rustc-link-search=native={}/libogc/lib/wii", dkp_path);
    println!("cargo:rustc-link-lib=static=c");
    println!("cargo:rustc-link-lib=static=sysbase");
    println!("cargo:rustc-link-lib=static=ogc");
}
