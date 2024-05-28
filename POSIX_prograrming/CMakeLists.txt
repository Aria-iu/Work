cmake_minimum_required(VERSION 3.28)
project(POSIX_prograrming C)

set(CMAKE_C_STANDARD 23)

add_executable(POSIX_prograrming main.c
        POSIX/print_epoch.c
        POSIX/P_total.h
        POSIX/P_trace.c
        POSIX/P_locale.c
        POSIX/P_a64l.c)
