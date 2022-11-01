import module_dataurl from "./module_bytes.mjs";

// Make a setter / getter for handling network traffic:
let ssl_send;
let ssl_recv;
export function set_ssl_send(func) {
	ssl_send = func;
}
export function set_ssl_recv(func) {
	ssl_recv = func;
}

export const memory = new WebAssembly.Memory({
	initial: 10 // What should this actually be set too?
});
let mbedtls_ssl_handshake;
const timers = new Map();
const imports = {
	env: {
		memory,
		time(dest) {
			// Date.now gives us the number of *miliseconds* since January 1, 1970
			const secs = Math.trunc(Date.now() / 1000);
			if (dest != 0) {
				// Write secs to the pointer:
				const view = new DataView(memory.buffer);
				view.setBigInt64(dest, BigInt(secs), true);
			}
			return secs;
		},
		send(ctx, ptr, len) {
			const packet = new Uint8Array(memory.buffer, ptr, len);
			if (ssl_send) {
				return ssl_send(ctx, ptr);
			}
			return 0;
		},
		recv(ctx, ptr, len) {
			const dest_buf = new Uint8Array(memory.buffer, ptr, len);
			if (ssl_recv) {
				return ssl_recv(ctx, dest_buf);
			}
			return 0;
		},
		set_timer(ssl, int_ms, fin_ms) {
			const old_timeout = timers.get(ssl);
			if (old_timeout) {
				clearTimeout(old_timeout.handle);
			}
			if (fin_ms === 0) {
				timers.delete(ssl);
			} else {
				const now = Date.now();
				timers.set(ssl, {
					handle: setTimeout(() => {
						const res = mbedtls_ssl_handshake(ssl);
						console.log("Timeout triggered a handshake: " + res.toString(16));
					}, fin_ms),
					fin: now + fin_ms,
					int: now + int_ms
				});
			}
		},
		get_timer(ssl) {
			const timer = timers.get(ssl);
			if (!timer) {
				return -1;
			} else {
				const { fin, int } = timer;
				const now = Date.now();
				if (now > fin) {
					return 2;
				} else if (now > int) {
					return 1;
				} else {
					return 0;
				}
			}
		},
		rand(ptr, len) {
			crypto.getRandomValues(new Uint8Array(memory.buffer, ptr, len));
		}
	}
};
const response = await fetch(module_dataurl);
const {module, instance} = await WebAssembly.instantiateStreaming(response, imports);
mbedtls_ssl_handshake = instance.exports.mbedtls_ssl_handshake;

export default instance.exports;
export const wasm_module = module;
