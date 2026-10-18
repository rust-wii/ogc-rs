extern crate bindgen;

use bindgen::callbacks::ParseCallbacks;
use regex::Regex;
use std::env;
use std::process::Command;
use std::path::{Path, PathBuf};

fn get_include_path(dkp_path: &str, dkppc_path: &str) -> Vec<String>{
	let mut include = Vec::new();
	//powerpc-eabi-gcc -xc -E -v /dev/null
	let gcc_output = match Command::new(format!("{dkppc_path}/bin/powerpc-eabi-gcc"))
		.arg("-xc")
		.arg("-E")
		.arg("-v")
		.arg("/dev/null").output() {
		Ok(output) => output,
		Err(e) => panic!("failed to get the default include paths on the host machine!\n{}", e),
	};
	let output = gcc_output.stderr;
	
	let parsed_output =
		String::from_utf8(output).expect("gcc command output returned a non-utf8 string.");
	parsed_output.split("\n").filter(|line| line.trim().starts_with(dkp_path) && line.contains("include")).for_each(|line| {
		include.push(line.trim().to_string());
	});
	include
}

fn get_clang_version() -> String {
	// Check if the clang version env variable exists.
	if env::var("CLANG_VERSION").is_err() {
		// Attempt to retrieve clang version through the command line.
		let clang_output = match Command::new("clang").arg("--version").output() {
			Ok(output) => output,
			Err(_e) => panic!("Could not find clang on the host machine!"),
		};

		// Get the first line of the output, usually containing the version string.
		let output = clang_output.stdout;
		let parsed_output =
			String::from_utf8(output).expect("Clang command output returned a non-utf8 string.");
		let first_line = match parsed_output.lines().next() {
			Some(line) => line,
			None => panic!("Clang command output does not contain split lines."),
		};

		// Parse the version string using Regex.

		let regex = Regex::new(r"(?m)\d+(?:\.\d+)+").unwrap();
		let result = regex.captures(first_line).unwrap().get(0); // Attempt to join together the version string.

		let version = match result {
			Some(v) => v.as_str(),
			None => {
				panic!("Failed to parse version, please export your clang version to CLANG_VERSION")
			}
		};

		// Return the final joined string.
		version.to_string()
	} else {
		// Clang version env variable exists, use that over parsing.
		env::var("CLANG_VERSION").unwrap()
	}
}

fn main() {
	let dkp_path = env::var("DEVKITPRO").expect("The devkitPRO toolchain is required to use this crate; please verify that your environment variables are correctly configured");
	let dkppc_path = env::var("DEVKITPPC").expect("The devkitPPC toolchain is required to use this crate; please verify that your environment variables are correctly configured");
	
	// DEVKITPPC may contain a Windows path (D:/devkitPro/devkitPPC), but MSYS2
	// PATH uses Unix-style drive paths (/d/devkitPro/devkitPPC/bin). Using the
	// wrong format can make powerpc-eabi-gcc unavailable even if installed.
	//
	// Convert only the displayed fix; DEVKITPPC remains unchanged for compiler,
	// include, and linker paths. This check belongs here because ogc-rs is the
	// first build step that directly requires devkitPPC to generate bindings.
	let path_hint = if dkppc_path.len() > 2 && dkppc_path.as_bytes()[1] == b':' {
		format!("/{}/{}", dkppc_path[..1].to_lowercase(), &dkppc_path[3..])
	} else {
		dkppc_path.clone()
	};

	if Command::new(format!("{dkppc_path}/bin/powerpc-eabi-gcc")).arg("--version").output().is_err() {
		panic!(
			"powerpc-eabi-gcc was not found.\n\
			devkitPPC's executables are required to be available in PATH.\n\
			$DEVKITPPC is set, but the compiler cannot be found.\n\
			This usually means the PATH entry is missing.\n\
			\n\
			Add this to a startup configuration file like ~/.bashrc:\n\
			export PATH=\"{}/bin:$PATH\"\n\
			\n\
			\n\
			After changing the PATH, restart your terminal or reload your shell.",
			path_hint
		);
	}
	println!(
		"cargo:rustc-link-search=native={}/powerpc-eabi/lib",
		dkppc_path
	);
	println!("cargo:rustc-link-search=native={}/libogc/lib/wii", dkp_path);

	println!("cargo:rustc-link-lib=static=sysbase");
	println!("cargo:rustc-link-lib=static=c");
	println!("cargo:rustc-link-lib=static=m");
	println!("cargo:rustc-link-lib=static=db");
	println!("cargo:rustc-link-lib=static=ogc");
	println!("cargo:rustc-link-lib=static=asnd");
	println!("cargo:rustc-link-lib=static=mad");
	println!("cargo:rustc-link-lib=static=aesnd");

	//MP3Player

	//Wiipad
	println!("cargo:rustc-link-lib=static=bte");
	println!("cargo:rustc-link-lib=static=wiiuse");

	println!("cargo:rerun-if-changed=wrapper.h");
	#[derive(Debug)]
	struct CBParser;
	impl ParseCallbacks for CBParser {
		fn process_comment(&self, comment: &str) -> Option<String> {
			Some(doxygen_rs::transform(comment))
	  }
	  fn header_file(&self, filename: &str) {
			println!("cargo:rerun-if-changed={}", filename);
	  }
 
	  fn include_file(&self, filename: &str) {
			println!("cargo:rerun-if-changed={}", filename);
	  }
 
	  fn read_env_var(&self, key: &str) {
			println!("cargo:rerun-if-env-changed={}", key);
	  }
	}
	let mut bindings = bindgen::Builder::default()
		.header("wrapper.h")
		.rust_target(bindgen::RustTarget::nightly())
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
		.clang_arg(format!(
			"-isystem{}/powerpc-eabi/include",
			dkppc_path
		))
		.clang_arg(format!(
			"-isystem/usr/lib/clang/{}/include",
			get_clang_version()
		));

		let includes = get_include_path(&dkp_path, &dkppc_path);
		includes.iter().for_each(|include| {
			bindings = bindings.clone().clang_arg(format!("-I{}", include));
		});

		// libogc is not always installed with the devkitPro toolchain,
		// so check for it before generating bindings to avoid confusing build failures.
		let libogc_include = Path::new(&dkp_path).join("libogc/include");

		if !libogc_include.exists() {
			panic!("libogc is not installed.");
		}
		
		let bindings = bindings.clang_arg(format!("-I{}", libogc_include.display()))
		.clang_arg("-mfloat-abi=hard")
		.clang_arg("-nostdinc")
		.clang_arg("-Wno-macro-redefined")
		.clang_arg("-Wno-incompatible-library-redeclaration")
		.clang_arg("-DHW_RVL")
		.parse_callbacks(Box::new(CBParser))
		.generate()
		.expect("Unable to generate bindings");

	let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
	bindings
		.write_to_file(out_path.join("bindings.rs"))
		.expect("Unable to write bindings to file");
}
