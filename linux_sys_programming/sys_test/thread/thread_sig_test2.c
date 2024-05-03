//
// Created by zyc on 24-5-3.
//
#include <stdio.h>
#include <stdlib.h>
#include <pthread.h>
#include <signal.h>
#include <unistd.h>

#define NUM_THREADS 3

// 全局变量，用于表示线程是否应该退出
volatile sig_atomic_t should_exit = 0;

// 线程函数
void *thread_function(void *arg) {
    while (!should_exit) {
        // 线程执行的任务
        printf("Thread %ld is running...\n", pthread_self());
        sleep(10);
    }
    printf("Thread %ld is exiting...\n", pthread_self());
    pthread_exit(NULL);
}

// 信号处理函数
void signal_handler(int signum) {
    printf("Signal received by main thread: %d\n", signum);
    // 将退出标志设置为真
    should_exit = 1;
}

int main() {
    pthread_t threads[NUM_THREADS];

    // 设置信号处理函数
    signal(SIGUSR1, signal_handler);

    // 创建线程
    for (int i = 0; i < NUM_THREADS; i++) {
        if (pthread_create(&threads[i], NULL, thread_function, NULL) != 0) {
            perror("pthread_create");
            exit(EXIT_FAILURE);
        }
    }

    // 主线程等待信号
    printf("Main thread %d is waiting for signal...\n",getpid());
    pause(); // 暂停主线程，等待信号

    // 发送信号给所有线程
    for (int i = 0; i < NUM_THREADS; i++) {
        pthread_kill(threads[i], SIGUSR1);
    }

    // 等待所有线程结束
    for (int i = 0; i < NUM_THREADS; i++) {
        if (pthread_join(threads[i], NULL) != 0) {
            perror("pthread_join");
            exit(EXIT_FAILURE);
        }
    }

    printf("Main thread exits.\n");
    return 0;
}
