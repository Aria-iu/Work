//
// Created by zyc on 24-5-8.
//
#include "P_total.h"
#include <time.h>

void P_printEpoch() {
    // 声明一个 time_t 类型的变量来存储时间戳
    time_t timestamp;

    // 使用 time 函数获取当前时间戳
    timestamp = time(NULL);

    // 打印当前时间戳
    printf("Current timestamp: %ld\n", timestamp);

}

