/**************************************************************************/
/* Rust Build File build.rs                                               */
/**************************************************************************/

use bindgen::MacroTypeVariation;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::{env, fs, path::Path};

fn main() -> std::io::Result<()> {
	println!("cargo:rerun-if-changed=cvi/ui.h");

	let input_path = "cvi/ui.h";
	let output_path = "cvi/uiBind.h";
	let bindings_out_path = "src/ui.rs";

	// Handle the Result properly here
	let input_file = File::open(input_path)?;
	let reader = BufReader::new(input_file);

	let mut output_file = File::create(output_path)?;

	for line in reader.lines() {
		let line = line?;
		if line.trim_start().starts_with("#include")
			|| line.trim_start().starts_with("int  CVICALLBACK")
			|| line.trim_start().starts_with("void CVICALLBACK")
		{
			continue;
		}
		writeln!(output_file, "{}", line)?;
	}

	println!("Filtered content written to {}", output_path);

	let bindings = bindgen::Builder::default()
		.header(output_path)
		.default_macro_constant_type(MacroTypeVariation::Signed) // to i32
		.generate()
		.expect("Unable to generate bindings");

	bindings.write_to_file(bindings_out_path).expect("Couldn't write bindings!");

	// Read the file, prepend the attribute, and write it back
	let mut contents = std::fs::read_to_string(bindings_out_path)?;
	contents = format!("#![allow(dead_code)]\n{}", contents);
	std::fs::write(bindings_out_path, contents)?;

	//--------------------------------------------------------------
	// UIR File Copy
	// Ensure build script reruns if the source file changes
	println!("cargo:rerun-if-changed=cvi/UI.uir");

	// Get the output directory (e.g., target/debug or target/release)
	let out_dir = env::var("OUT_DIR").unwrap();
	// OUT_DIR is something like .../target/debug/build/yourcrate-xxxxxx/out
	// To get to target/debug or target/release:
	let _profile = env::var("PROFILE").unwrap();
	let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
	let target_dir = Path::new(&out_dir)
		.ancestors()
		.nth(3) // Go up 3 levels: out -> build -> debug/release -> target
		.expect("Failed to determine target directory");

	// Construct the destination path: target/debug/bin/subfolder/SpiderChart.uir
	//let dest_dir = target_dir.join(&profile).join("bin").join("subfolder");
	let dest_dir = target_dir.join("bin");
	fs::create_dir_all(&dest_dir).expect("Failed to create destination directory");

	let src_file = Path::new(&manifest_dir).join("cvi/UI.uir");
	let dest_file = dest_dir.join("UI.uir");
	fs::copy(&src_file, &dest_file).expect("Could not copy uir file");
	println!("src {:?}, dst {:?}", src_file, dest_file);

	// official ext compiler support
	println!("cargo:rustc-link-lib=lib\\cvirt");
	println!("cargo:rustc-link-lib=lib\\cvisupp");

	println!("cargo:rustc-link-arg=lib\\ui.obj");

	println!("cargo:rustc-link-lib=user32");

	Ok(())
}
