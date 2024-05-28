#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <sys/poll.h>
#include <sys/time.h>
#include <signal.h>


void signal_handler(int signum) {
    printf("Received signal: %d\n", signum);
}

int main() {
    struct pollfd fds[1];
    int ret;

    // 设置信号处理函数
    signal(SIGINT, signal_handler);

    // 准备监视的文件描述符
    fds[0].fd = STDIN_FILENO;
    fds[0].events = POLLIN;
/*
    // 使用poll()函数进行事件轮询
    printf("Using poll()...\n");
    ret = poll(fds, 1, 5000); // 等待5秒
    if (ret == -1) {
        perror("poll() error");
    } else if (ret == 0) {
        printf("Timeout occurred!\n");
    } else {
        if (fds[0].revents & POLLIN) {
            printf("Input is ready on stdin.\n");
            // 读取输入
            char buffer[256];
            fgets(buffer, sizeof(buffer), stdin);
            printf("Read: %s", buffer);
        }
    }
*/
    // 准备信号掩码
    sigset_t mask;
    sigemptyset(&mask);
    sigaddset(&mask, SIGINT);

    // 使用ppoll()函数进行事件轮询
    printf("\nUsing ppoll()...\n");
    ret = ppoll(fds, 1, NULL, &mask); // 等待信号
    if (ret == -1) {
        perror("ppoll() error");
    } else {
        if (fds[0].revents & POLLIN) {
            printf("Input is ready on stdin.\n");
            // 读取输入
            char buffer[256];
            fgets(buffer, sizeof(buffer), stdin);
            printf("Read: %s", buffer);
        } else if (fds[0].revents & POLLNVAL) {
            printf("Invalid request: the file descriptor is not open.\n");
        } else if (fds[0].revents & POLLERR) {
            printf("An error occurred on the file descriptor.\n");
        } else if (fds[0].revents & POLLHUP) {
            printf("The file descriptor hung up.\n");
        }
    }

    printf("Exiting...\n");
    return 0;
}
