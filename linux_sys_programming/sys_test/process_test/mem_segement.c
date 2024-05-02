#include "../total.h"

/*
 * 展示了linux下的可执行文件中程序变量在内存各段中的位置
 * gcc ./sys_test/process_test/mem_segement.c ./error_function.c -o mem_segement
 * */
char globalBuf[65535];
int primes[] = {2,3,5,7};

static int
square (int x){
    return x*x;
}

static void
doCalc(int val){
    printf("The square of %d is %d\n",val, square(val));
    if (val<1000){
        int t;
        t = val*val*val;
        printf("The cube of %d is %d\n",val, t);
    }
}

int
main(int argc,char* argv[]){
    static int key = 9973;
    static char mbuf[10240000];
    char *p;
    p = malloc(1024);
    doCalc(key);
    exit(EXIT_SUCCESS);
}