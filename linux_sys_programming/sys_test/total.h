#ifndef __TOTAL__
#define __TOTAL__

#include <stdio.h>
#include <stdlib.h>
#include <errno.h>
#include "../error_function.h"
#include <string.h>

void print_my_error_msg(char *msg,int errnum){
    printf("--------CAUSED ERROR HERE!!!------\n");
    printf(msg);
    printf("\n");
    printf("--------ENDED  ERROR HERE!!!------\n");
    // 发生错误，结束当前进程
    exit(EXIT_FAILURE);
}

#endif