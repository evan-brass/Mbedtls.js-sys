#include "mbedtls/ssl_cookie.h"
#include "imports.h"

// Cookie function pointers:
int (*get_cookie_write_ptr()) (void*, unsigned char **, unsigned char *, const unsigned char *, size_t) {
	return &mbedtls_ssl_cookie_write;
}
int (*get_cookie_check_ptr()) (void*, const unsigned char *, size_t, const unsigned char *, size_t) {
	return &mbedtls_ssl_cookie_check;
}

// Get function pointers for the delay imports:
void (*get_set_timer_ptr()) (void*, unsigned long, unsigned long) {
	return &set_timer;
}
int (*get_timer_ptr()) (void*) {
	return &get_timer;
}

// Get function pointers for the send and receive imports:
int (*get_send_ptr())(void*, const unsigned char *, size_t) {
	return &send;
}
int (*get_recv_ptr())(void*, const unsigned char *, size_t) {
	return &recv;
}

// Allocators
// TODO: 
