//
// Created by zyc on 24-5-6.
//
#include "../total.h"
#include <unistd.h>
#include <sys/wait.h>

int
main(int argc, char *argv[]){
    int pfd [2];
    if (pipe(pfd)==-1)
        errExit("pipe");

    switch (fork()) {
        case -1:
            errExit("fork");

        case 0:             /* First child: exec 'ls' to write to pipe */
            if (close(pfd[0]) == -1)                    /* Read end is unused */
                errExit("close 1");

            /* Duplicate stdout on write end of pipe; close duplicated descriptor */

            if (pfd[1] != STDOUT_FILENO) {              /* Defensive check */
                if (dup2(pfd[1], STDOUT_FILENO) == -1)
                    errExit("dup2 1");
                if (close(pfd[1]) == -1)
                    errExit("close 2");
            }

            execlp("ls", "ls", (char *) NULL);          /* Writes to pipe */
            errExit("execlp ls");

        default:            /* Parent falls through to create next child */
            break;
    }
    /*
     *
     * */
    switch (fork()) {
        case -1:
            errExit("fork");
        case 0:
            if (close(pfd[1])==-1)
                errExit("close 3");
            if (pfd[0]!=STDIN_FILENO){
                if (dup2(pfd[0],STDIN_FILENO)==-1)
                    errExit("dup2 2");
                if (close(pfd[0])==-1)
                    errExit("close 4");
            }
            execlp("wc","wc","-l",(char *)NULL);
            errExit("execlp wc");
        default:
            break;
    }
    if (close(pfd[0])==-1)
        errExit("close 5");
    if (close(pfd[1])==-1)
        errExit("close 6");
    if (wait(NULL)==-1)
        errExit("wait 1");
    if (wait(NULL)==-1)
        errExit("wait 2");
    exit(EXIT_SUCCESS);

}