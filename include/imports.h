// These are functions which we import from the JavaScript side:
#include <time.h>

__attribute__((import_name("time"))) time_t time(time_t *);
