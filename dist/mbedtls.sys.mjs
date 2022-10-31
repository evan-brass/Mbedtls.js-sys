import module_dataurl from "./module_bytes.mjs";
import make_imports from "./imports.mjs";


export const memory = new WebAssembly.Memory({
	initial: 10 // What should this actually be set too?
});
const imports = make_imports(memory);
const response = await fetch(module_dataurl);
const {module, instance} = await WebAssembly.instantiateStreaming(response, imports);

export default instance.exports;
export const wasm_module = module;
