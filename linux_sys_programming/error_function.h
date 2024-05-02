#ifndef ERROR_FUNCTION_H
#define ERROR_FUNCTION_H

/*
 * 使用 errMsg()、errExit()、err_exit()以及 errExitEN()函数，
 * 以诊断调用系统调用和库函数时所发生的错误。
 *
 *
 * */

// errMsg()会在标准错误设备上打印消息
// 打印出与当前 errno 值相对应的错误文本，
// 其中包括了错误名（比如，EPERM）以及由 strerror()返回的错误描述，外加
// 由参数列表指定的格式化输出。
void errMsg(const char*format,...);

#ifdef __GNUC__
#define NORETURN __attribute__ ((__noreturn__))
#else
#define NORETURN
#endif

// errExit()函数的操作方式与 errMsg()相似,还会以如下两种方式之一来终止程序
// 其一，调用 exit()退出。
// 其二，若将环境变量 EF_DUMPCORE 定义为非空字符串，则调用 abort()退出，
//  同时生成核心转储（core dump）文件，供调试器调试之用。
void errExit(const char*format, ... ) NORETURN;

// 函数 err_exit()类似于 errExit()
// 打印错误消息之前，err_exit()不会刷新标准输出。
// err_exit()终止进程使用的是_exit()，而非 exit()。这一退出方式，略去了对 stdio 缓冲区
// 的刷新以及对退出处理程序（exit handler）的调用
void err_exit(const char*format, ... ) NORETURN;

// 功能上，errExitEN()函数与 errExit()大体相同，区别仅仅在于：与 errExit()打印与当前
// errno 值相对应的错误文本不同，errExitEN()只会打印与 errnum 参数中给定的错误号相对应的文本。
void errExitEN(int errnum,const char*format,...) NORETURN;

// 诊断其他类型的错误时，使用的是 fatal()、usageErr()以及 cmdLineErr()。

// fatal()用来诊断一般性错误，其中包括未设置 errno 的库函数错误。
// 该函数会在标准错误上打印格式化输出，然后，像 errExit()那样终止程序。
void fatal(const char* format,...) NORETURN;

// usageErr()用来诊断命令行参数使用方面的错误。
// 参数列表风格与 printf()相同，并在
// 标准错误上打印字符串“Usage：”，随之以格式化输出，然后调用 exit()终止程序。
void usageErr(const char* format,...) NORETURN;

// 函数 cmdLineErr()酷似 usageErr()，但其错误诊断是针对于特定程序的命令行参数。
void cmdLineErr(const char *format,...) NORETURN;

#endif