#ifndef _IMPORTS_H
#define _IMPORTS_H

// These are functions which we import from the JavaScript side:
#include <time.h>
#include <stddef.h>

__attribute__((import_name("time"))) time_t time(time_t *);
__attribute__((import_name("rand"))) int rand(unsigned char *, size_t);

int rng_func(void *, unsigned char *, size_t);
int (*get_rng_ptr()) (void*, unsigned char *, size_t);

#endif
