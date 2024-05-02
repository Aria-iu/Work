//
// Created by zyc on 24-5-1.
//
#include "../total.h"
#include <fcntl.h>
#include <sys/uio.h>
/*
 * gcc ./sys_test/file_io_test/readv.c error_function.c -o readv
 *
 * */
int
main(int argc,char *argv[])
{
    int fd;
    struct iovec iov[3];
    struct stat myStruct;
    int x;
#define STR_SIZE 100
    char str[STR_SIZE];
    ssize_t numRead,totRequested;

    if (argc != 2 || strcmp(argv[1],"--help")==0)
        usageErr("%s file\n",argv[0]);

    fd = open(argv[1],O_RDONLY);
    if (fd == -1)
        errExit("open");

    totRequested = 0;

    iov[0].iov_base = &myStruct;
    iov[0].iov_len = sizeof(struct stat);
    totRequested += iov[0].iov_len;

    iov[1].iov_base = &x;
    iov[1].iov_len = sizeof(x);
    totRequested += iov[1].iov_len;

    iov[2].iov_base = str;
    iov[2].iov_len = STR_SIZE;
    totRequested += iov[2].iov_len;

    numRead = readv(fd,iov,3);
    if (numRead==-1)
        errExit("readv");

    if (numRead<totRequested)
        printf("Read fewer bytes than requested\n");

    printf("total bytes requested : %ld; bytes read: %ld \n",
           (long)totRequested,(long)numRead);
    exit(EXIT_FAILURE);
}