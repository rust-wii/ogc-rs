use std::process::Command;
fn main() {
	let dkp_path = std::env::var("DEVKITPRO").expect("Please set $DEVKITPRO");
	println!("cargo:rustc-link-search=native={}/libogc/lib/wii", dkp_path);

	//checks if the build folder exists. If it does, it deletes it.
	let _ = std::fs::remove_dir_all("build");

	let _ = std::fs::create_dir("build");


	let libgcc_location = match Command::new("powerpc-eabi-gcc").arg("-print-libgcc-file-name").output() {
		Ok(output) => output,
		Err(_e) => panic!("Could not find powerpc-eabi-gcc or the libgcc on the host machine!"),
	};
	let output = libgcc_location.stdout;
	let parsed_output =
		String::from_utf8(output).expect("powerpc-eabi-gcc command output returned a non-utf8 string.").replace("\n", "");
		
	let _ = match Command::new("powerpc-eabi-ar").arg("x").arg(parsed_output).arg("crtresxfpr.o").arg("crtresxgpr.o").output() {
		Ok(output) => output,
		Err(_e) => panic!("powerpc-eabi-ar command failed"),
	};

	std::fs::rename("crtresxgpr.o", "build/crtresxgpr.o").expect("Could not move crtresxgpr.o");
	std::fs::rename("crtresxfpr.o", "build/crtresxfpr.o").expect("Could not move crtresxfpr.o");

	println!("cargo::rustc-link-arg=build/crtresxgpr.o");
	println!("cargo::rustc-link-arg=build/crtresxfpr.o");
}
