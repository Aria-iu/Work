//
// Created by zyc on 24-5-3.
//
#include <stdio.h>
#include <stdlib.h>
#include <pthread.h>
#include <signal.h>
#include <unistd.h>

#define NUM_THREADS 3

// 共享资源
int shared_data = 0;

// 互斥量
pthread_mutex_t mutex = PTHREAD_MUTEX_INITIALIZER;

// 信号处理函数
void signal_handler(int signum) {
    printf("Signal received: %d\n", signum);
}

// 线程函数
void *thread_function(void *arg) {
    // 阻塞所有信号
    sigset_t signal_set;
    sigfillset(&signal_set);
    pthread_sigmask(SIG_BLOCK, &signal_set, NULL);

    // 等待信号
    int sig;
    while (1) {
        sigwait(&signal_set, &sig);
        // 处理信号
        pthread_mutex_lock(&mutex);
        shared_data++;
        printf("Thread %ld incremented shared_data to %d\n", pthread_self(), shared_data);
        pthread_mutex_unlock(&mutex);
        break;
    }
    return NULL;
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

    // 发送信号
    sleep(1);
    printf("Sending signal...\n");
    pthread_kill(threads[0], SIGUSR1);
    pthread_kill(threads[1], SIGUSR1);
    pthread_kill(threads[2], SIGUSR1);
    // 等待线程结束
    for (int i = 0; i < NUM_THREADS; i++) {
        if (pthread_join(threads[i], NULL) != 0) {
            perror("pthread_join");
            exit(EXIT_FAILURE);
        }
    }

    return 0;
}
