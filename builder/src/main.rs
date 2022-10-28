use eyre::{Result, eyre};
use std::io::Write;
use std::path::Path;
use std::process::Command;

fn main() -> Result<()> {
	// Create the build folder:
	let _ = std::fs::create_dir("build");

	// Compile all the c files:
	let paths = glob::glob("src/**/*.c")?;

	let mut bail = false;
	for p in paths {
		let in_path = p?;
		let mut out_path = Path::new("build").join(in_path.file_stem().ok_or(eyre!("wat"))?);
		out_path.set_extension("o");

		let status = Command::new("clang")
			.args([
				"--target=wasm32-unknown-unknown",
				"-Iinclude",
				"-Ivendor/mbedtls/include",
				// "-Ivendor/musl/include",
				"-DMBEDTLS_CONFIG_FILE=\"wasm_config.h\"",
				"-c",
				"-o"
			])
			.arg(out_path)
			.arg(&in_path)
			.status()?;
		if !status.success() { bail = true; break; }
		println!("{in_path:?} {status}");
	}
	if bail {
		return Err(eyre!("Build failed"))
	}

	// Create the dist folder:
	let _ = std::fs::create_dir("dist");

	// Link everything together into a wasm dynamic library:
	let objects = glob::glob("build/*.o")?;
	let mut files = Vec::new();
	for path in objects {
		files.push(path?);
	}
	let status = Command::new("wasm-ld")
		.args([
			"--export-all",
			"--no-entry",
			"-o",
			"dist/mbedtls.wasm"
		])
		.args(files)
		.status()?;
	if !status.success() {
		return Err(eyre!("Linking failed."))
	}

	// Embed the wasm modules's bytes into a JavaScript module to make it easier to import in Deno/WebBrowser:
	let wasm_file = std::fs::File::open("dist/mbedtls.wasm")?;
	let mut wasm_file = base64_stream::ToBase64Reader::new(wasm_file);
	let mut module_bytes_file = std::fs::File::create("dist/module_bytes.mjs")?;
	module_bytes_file.write_all(b"export default await fetch(\"data:application/wasm;base64,")?;
	std::io::copy(&mut wasm_file, &mut module_bytes_file)?;
	module_bytes_file.write_all(b"\");")?;

	Ok(())
}
