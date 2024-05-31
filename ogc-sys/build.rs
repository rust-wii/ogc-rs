extern crate bindgen;

use bindgen::callbacks::ParseCallbacks;
use regex::Regex;
use std::env;
use std::process::Command;

fn get_include_path(dkp_path: String) -> Vec<String>{
	let mut include = Vec::new();
	//powerpc-eabi-gcc -xc -E -v /dev/null
	let gcc_output = match Command::new("powerpc-eabi-gcc")
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
	parsed_output.split("\n").filter(|line| line.trim().starts_with(&dkp_path) && line.contains("include")).for_each(|line| {
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
	if std::env::var("DOCS_RS").is_ok() {
		return;
	}
	let dkp_path = env::var("DEVKITPRO").expect("devkitPro is needed to use this crate");
	println!(
		"cargo:rustc-link-search=native={}/devkitPPC/powerpc-eabi/lib",
		dkp_path
	);
	println!("cargo:rustc-link-search=native={}/libogc/lib/wii", dkp_path);

	println!("cargo:rustc-link-lib=static=c");
	println!("cargo:rustc-link-lib=static=sysbase");
	println!("cargo:rustc-link-lib=static=m");
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
		.clang_arg(format!("--sysroot={}/devkitPPC/powerpc-eabi", dkp_path))
		.clang_arg(format!(
			"-isystem{}/devkitPPC/powerpc-eabi/include",
			dkp_path
		))
		.clang_arg(format!(
			"-isystem/usr/lib/clang/{}/include",
			get_clang_version()
		));

		let includes = get_include_path(dkp_path.clone());
		includes.iter().for_each(|include| {
			bindings = bindings.clone().clang_arg(format!("-I{}", include));
		});


		let bindings = bindings.clang_arg(format!("-I{}/libogc/include", dkp_path))
		.clang_arg("-mfloat-abi=hard")
		.clang_arg("-nostdinc")
		.clang_arg("-Wno-macro-redefined")
		.clang_arg("-Wno-incompatible-library-redeclaration")
		.clang_arg("-DHW_RVL")
		.parse_callbacks(Box::new(CBParser))
		.generate()
		.expect("Unable to generate bindings");

	bindings
		.write_to_file("./src/ogc.rs")
		.expect("Unable to write bindings to file");
}
