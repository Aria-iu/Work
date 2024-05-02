//
// Created by zyc on 24-5-2.
//

#ifndef LINUX_SYS_PROGRAMMING_SIG_FUNCTIONS_H
#define LINUX_SYS_PROGRAMMING_SIG_FUNCTIONS_H


#include <signal.h>
#include "../total.h"

int printSigMask(FILE *of, const char *msg);

int printPendingSigs(FILE *of, const char *msg);

void printSigset(FILE *of, const char *ldr, const sigset_t *mask);


#endif //LINUX_SYS_PROGRAMMING_SIG_FUNCTIONS_H
