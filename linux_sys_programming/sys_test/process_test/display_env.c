//
// Created by zyc on 24-5-1.
//

/*
 * 展示了linux下的进程环境变量
 * gcc error_function.c ./sys_test/process_test/display_env.c -o display_env
 * */
#include "../total.h"

extern char ** environ;

int
main(int argc,char *argv[]){
    char **ep;

    for (ep=environ;*ep!=NULL;ep++)
        puts(*ep);

    exit(EXIT_SUCCESS);
}