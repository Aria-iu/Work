//
// Created by zyc on 24-5-6.
//
#include "../total.h"
#include <unistd.h>
#include "../../get_num.h"


int
main(int argc, char *argv[])
{
    int pfd[2];                             /* Process synchronization pipe */
    int j, dummy;

    if (argc < 2 || strcmp(argv[1], "--help") == 0)
        usageErr("%s sleep-time...\n", argv[0]);

    setbuf(stdout, NULL);                   /* Make stdout unbuffered, since we
                                               terminate child with _exit() */
    printf(" Parent started\n");

    if (pipe(pfd) == -1)
        errExit("pipe");

    printf("pipe[0] is %d,pipe[1] is %d\n", pfd[0],pfd[1]);

    for (j = 1; j < argc; j++) {
        switch (fork()) {
            case -1:
                errExit("fork %d", j);

            case 0: /* Child */
                if (close(pfd[0]) == -1)        /* Read end is unused */
                    errExit("close");

                /* Child does some work, and lets parent know it's done */

                sleep(getInt(argv[j], GN_NONNEG, "sleep-time"));
                /* Simulate processing */
                printf("Child %d (PID=%ld) closing pipe\n",
                        j, (long) getpid());
                if (close(pfd[1]) == -1)
                    errExit("close");

                /* Child now carries on to do other things... */

                _exit(EXIT_SUCCESS);

            default: /* Parent loops to create next child */
                break;
        }
    }

    /* Parent comes here; close write end of pipe so we can see EOF */

    if (close(pfd[1]) == -1)                /* Write end is unused */
        errExit("close");

    /* Parent may do other work, then synchronizes with children */

    if (read(pfd[0], &dummy, 1) != 0)
        fatal("parent didn't get EOF");
    printf("Parent ready to go\n");

    /* Parent can now carry on to do other things... */

    exit(EXIT_SUCCESS);
}