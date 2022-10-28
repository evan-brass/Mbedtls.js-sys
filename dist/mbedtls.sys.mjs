import response from "./module_bytes.mjs";
import imports from "./imports.mjs";

export const module = await WebAssembly.instantiateStreaming(response, imports);
