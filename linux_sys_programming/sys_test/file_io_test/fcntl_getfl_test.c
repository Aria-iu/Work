//
// Created by zyc on 24-5-1.
//
#include <fcntl.h>
#include "../total.h"

void getfl_test(int fd){
    int flags,accessMode;
    flags = fcntl(fd,F_GETFL);
    if (flags==-1)
        errExit("fcntl");

    if (flags & O_SYNC)
        printf("writes ars sync\n");

    accessMode = flags & O_ACCMODE;
    if (accessMode == O_WRONLY|| accessMode == O_RDWR)
        printf("file is writeable\n");

    flags |= O_APPEND;
    if (fcntl(fd,F_SETFL,flags)==-1)
        errExit("fcntl");
}