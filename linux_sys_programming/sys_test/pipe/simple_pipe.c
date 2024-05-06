//
// Created by zyc on 24-5-6.
//
#include <sys/wait.h>
#include "../total.h"
#include <unistd.h>
#define BUF_SIZE 10

int
main(int argc,char *argv[]){
    int pfd [2];
    char *msg = "it's a cold day bro";
    char buf[BUF_SIZE];

    ssize_t numRead;
    if (argc!=1){
        usageErr("%s\n",argv[0]);
    }

    if (pipe(pfd)==-1)
        errExit("pipe");

    switch (fork()) {
        case -1:
            errExit("fork");
        case 0:
            if (close(pfd[1])==-1)
                errExit("close - child");
            for(;;){
                numRead = read(pfd[0],buf,BUF_SIZE);
                if (numRead==-1)
                    errExit("read");
                if (numRead==0)
                    break;
                if (write(STDOUT_FILENO,buf,numRead)!=numRead)
                    fatal("child - partial/failed write");
            }

            write(STDOUT_FILENO,"\n",1);
            if (close(pfd[0])==-1)
                errExit("close");
            _exit(EXIT_SUCCESS);
        default:
            if (close(pfd[0])==-1)
                errExit("close - parent");
            if (write(pfd[1],msg, strlen(msg))!= strlen(msg))
                fatal("parent - partial/failed write.");

            if (close(pfd[1])==-1)
                errExit("close");
            wait(NULL);
            exit(EXIT_SUCCESS);
    }
}