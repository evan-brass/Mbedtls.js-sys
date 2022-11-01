export default function make_imports(memory) {
	return {
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
			rand(ptr, len) {
				crypto.getRandomValues(new Uint8Array(memory.buffer, ptr, len));
			}
		}
	};
}
