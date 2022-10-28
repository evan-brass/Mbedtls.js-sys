#ifndef	_TIME_H
#define _TIME_H

#ifdef __cplusplus
extern "C" {
#endif

#include <stddef.h>

typedef long long time_t;
typedef void * locale_t;

struct tm {
	int tm_sec;
	int tm_min;
	int tm_hour;
	int tm_mday;
	int tm_mon;
	int tm_year;
	int tm_wday;
	int tm_yday;
	int tm_isdst;
	long __tm_gmtoff;
	const char *__tm_zone;
};

// clock_t clock (void);
time_t time (time_t *);
// double difftime (time_t, time_t);
// time_t mktime (struct tm *);
// size_t strftime (char *__restrict, size_t, const char *__restrict, const struct tm *__restrict);
struct tm *gmtime (const time_t *);
// struct tm *localtime (const time_t *);
// char *asctime (const struct tm *);
// char *ctime (const time_t *);
// int timespec_get(struct timespec *, int);

// #define CLOCKS_PER_SEC 1000000L

// #define TIME_UTC 1

#ifdef __cplusplus
}
#endif


#endif
