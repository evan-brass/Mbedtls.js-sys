use eyre::{Result, eyre};
use std::path::PathBuf;

fn main() -> Result<()> {
	let mut files = Vec::new();
	for p in glob::glob("c_src/**/*.c")? {
		files.push(p?);
	}

	cc::Build::new()
		.compiler("clang")
		.archiver("llvm-ar")
		.target("wasm32-unknown-unknown")
		.includes([
			"include",
			"vendor/mbedtls/include"
		])
		.define("MBEDTLS_CONFIG_FILE", "\"wasm_config.h\"")
		.files(files)
		.compile("mbedtls");

	let bindings = bindgen::builder()
		.header("include/bindgen.h")
		.clang_arg("-Iinclude")
		.clang_arg("-Ivendor/mbedtls/include")
		.clang_args([
			"-Iinclude",
			"-Ivendor/mbedtls/include",
			"-fvisibility=default",
			// "-DMBEDTLS_CONFIG_FILE=\"wasm_config.h\""
		])
		.layout_tests(false)
		.parse_callbacks(Box::new(bindgen::CargoCallbacks))
		.generate()?;
	let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap());
	bindings.write_to_file(out_path.join("mbedtls.rs"))?;

	Ok(())
}
