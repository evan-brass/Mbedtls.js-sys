#ifndef _IMPORTS_H
#define _IMPORTS_H

// These are functions which we import from the JavaScript side:
#include <time.h>
#include <stddef.h>

__attribute__((import_name("time"))) time_t time(time_t *);
__attribute__((import_name("rand"))) int rand(unsigned char *, size_t);
__attribute__((import_name("set_timer"))) void set_timer(void*, unsigned long, unsigned long);
__attribute__((import_name("get_timer"))) int get_timer(void*);

#endif
