#include "mbedtls/ssl_cookie.h"

// Cookie function pointers:
int (*get_cookie_write_ptr()) (void*, unsigned char **, unsigned char *, const unsigned char *, size_t) {
	return &mbedtls_ssl_cookie_write;
}
int (*get_cookie_check_ptr()) (void*, const unsigned char *, size_t, const unsigned char *, size_t) {
	return &mbedtls_ssl_cookie_check;
}

// Allocators
// TODO: 
