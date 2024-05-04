fn main() {
    let dkp_path = std::env::var("DEVKITPRO").expect("Please set $DEVKITPRO");
    println!("cargo:rustc-link-search=native={}/libogc/lib/wii", dkp_path);
    println!(
        "cargo:rustc-link-search=native={}/devkitPPC/powerpc-eabi/lib",
        dkp_path
    );
    println!("cargo:rustc-link-lib=static=gcc");
    println!("cargo:rustc-link-lib=static=sysbase");
    println!("cargo:rustc-link-arg=-T{}/devkitPPC/powerpc-eabi/lib/rvl.ld", dkp_path);
    
}