#include <gnu/libc-version.h>
#include "total.h"

void test_glibc_version(){
    char * version = gnu_get_libc_version();
    printf("this is version %s!\n",version);
}
