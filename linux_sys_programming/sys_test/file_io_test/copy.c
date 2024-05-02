//
// Created by zyc on 24-5-1.
//
#include "../total.h"
#include <sys/stat.h>
#include <fcntl.h>
#include <sys/types.h>
#include <unistd.h>

#ifndef BUF_SIZE
#define BUF_SIZE 1024
#endif

// ./copy oldfile newfile
// gcc sys_test/file_io_test/copy.c ./error_function.c -I./error_function.h -o copy
int
main(int argc,char *argv[]){
    int inputfd,outputfd,openflags;
    mode_t filePerms;
    ssize_t numRead;
    char buf[BUF_SIZE];

    if (argc!=3 || strcmp(argv[1],"--help")==0)
        usageErr("%s old-file new file\n",argv[0]);

    inputfd = open(argv[1],O_RDONLY);
    if (inputfd==-1)
        errExit("opening file %s",argv[1]);

    openflags = O_CREAT|O_WRONLY|O_TRUNC;
    filePerms = S_IRUSR|S_IRGRP|S_IWUSR|S_IWGRP|S_IROTH|S_IWOTH; // rw-rw-rw-
    outputfd = open(argv[2],openflags,filePerms);
    if (outputfd==-1)
        errExit("opening file %s",argv[2]);

    while((numRead=read(inputfd,buf,BUF_SIZE))>0)
        if(write(outputfd,buf,numRead)!=numRead)
            fatal("cannot write whole buffer");

    if (numRead==-1)
        errExit("read");
    if (close(inputfd)==-1)
        errExit("close input");
    if (close(outputfd)==-1)
        errExit("close output");

    exit(EXIT_FAILURE);
}