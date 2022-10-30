#include <stdlib.h>
#include <time.h>

long long __tm_to_secs(const struct tm *);
int __secs_to_tm(long long, struct tm *);
struct tm *__gmtime_r(const time_t *restrict t, struct tm *restrict tm);


extern const char __utc[];
