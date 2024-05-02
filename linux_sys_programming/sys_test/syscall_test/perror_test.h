#include <fcntl.h>
#include "../total.h"
#include <string.h>
/*
 * strerror根据错误号生成信息，若无法识别这个错误号，则返回
 * “Unknown error num.”
 * */
void perror_test(char *pathname,int flags,int mode){
    int fd = open(pathname,flags,mode);
    if (fd == -1){
        char * msg = strerror(errno);
        print_my_error_msg(msg,errno);
    }
}