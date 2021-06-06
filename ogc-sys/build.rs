extern crate bindgen;

fn main() {
    let dkp_path = std::env::var("DEVKITPRO").expect("devkitPro is needed to use this crate");
    let dkppc_path = std::env::var("DEVKITPPC").expect("devkitPro's devkitPPC is needed to use this crate");
    let clang_version = std::env::var("CLANG_VERSION").expect("You're clang version needs to be exported to CLANG_VERSION (ex. export CLANG_VERSION=11.0.0)");

    println!(
        "cargo:rustc-link-search=native={}/devkitPPC/powerpc-eabi/lib",
        dkp_path
    );
    println!("cargo:rustc-link-search=native={}/libogc/lib/wii", dkp_path);

    println!("cargo:rustc-link-lib=static=c");
    println!("cargo:rustc-link-lib=static=sysbase");
    println!("cargo:rustc-link-lib=static=ogc");

    println!("cargo:rerun-if-changed=wrapper.h");
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .rust_target(bindgen::RustTarget::Nightly)
        .use_core()
        .trust_clang_mangling(false)
        .layout_tests(false)
        .ctypes_prefix("::libc")
        .prepend_enum_name(false)
        .disable_untagged_union()
        .blocklist_type("u(8|16|32|64|128)")
        .blocklist_type("i(8|16|32|64|128)")
        .blocklist_type("f(32|64)")
        .clang_arg("--target=powerpc-none-eabi")
        .clang_arg(format!("--sysroot={}/powerpc-eabi", dkppc_path))
        .clang_arg(format!("-isystem/{}/powerpc-eabi/include", dkppc_path))
        .clang_arg(format!("-isystem/usr/lib/clang/{}/include", clang_version))
        .clang_arg(format!("-I{}/libogc/include", dkp_path))
        .clang_arg("-mfloat-abi=hard")
        .clang_arg("-nostdinc")
        .clang_arg("-Wno-macro-redefined")
        .clang_arg("-Wno-incompatible-library-redeclaration")
        .clang_arg("-DHW_RVL")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");
 
    bindings
        .write_to_file("./src/ogc.rs")
        .expect("Unable to write bindings to file");
}
