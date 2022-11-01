#include "imports.h"

int rng_func(void * _data, unsigned char * ptr, size_t len) {
	return rand(ptr, len);
}

int (*get_rng_ptr()) (void*, unsigned char *, size_t) {
	return &rng_func;
}
