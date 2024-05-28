//
// Created by zyc on 24-5-8.
//
// Linux 下没有实现sys/trace.h文件，但是提供了一个工具————strace来满足需求。
#include <stdio.h>
#include <unistd.h>
#include <sys/types.h>
#include <sys/wait.h>
/*
 * 在 Linux 中，可以使用 `strace` 工具来进行系统调用跟踪，而不是直接使用 POSIX 跟踪接口。`strace` 是一个强大的工具，可以追踪程序执行期间发生的系统调用，并输出相应的调用信息，例如调用类型、参数和返回值等。
 * 以下是一个简单的示例，演示如何使用 `strace` 工具来跟踪一个程序的执行：
 * 假设我们有一个名为 `my_program` 的可执行文件，我们想要跟踪它的执行：
 * ```
 * $ strace ./my_program
 * ```
 * 这样就会启动 `my_program` 并开始跟踪它的系统调用。`strace` 会输出每个系统调用的信息，包括调用类型、参数和返回值等。通过分析这些信息，可以更深入地了解程序的行为，并进行调试和性能优化。
 * 请注意，`strace` 工具并不是 POSIX 标准的一部分，而是 Linux 下的一个常用工具，因此你不需要担心找不到 `sys/trace.h` 头文件。
 *
 * */
// #include <sys/trace.h>
/*
int P_tracce() {
    pid_t pid;

    // 创建一个跟踪流
    int trace_stream = posix_trace_create(0);

    // 创建子进程
    pid = fork();

    if (pid < 0) {
        perror("fork");
        return 1;
    } else if (pid == 0) {
        // 子进程
        // 将子进程的pid注册到跟踪流中
        posix_trace_pid_join(trace_stream, getpid());
        printf("Child process\n");
        // 子进程执行一些操作
        // ...

        // 子进程结束，离开跟踪流
        posix_trace_pid_part(trace_stream, getpid());
        _exit(0);
    } else {
        // 等待子进程结束
        wait(NULL);
        // 父进程结束跟踪流
        posix_trace_close(trace_stream);
        printf("Parent process\n");
    }

    return 0;
}
 */