#ifndef _STRING_H_
#define _STRING_H_
#include <stddef.h>

size_t strlen(const char * str)
{
	size_t ret = 0;
	for (;str[ret]; ret++);
	return ret;
}

#endif
