cmake_minimum_required(VERSION 3.28)
project(linux_sys_programming C)

set(CMAKE_C_STANDARD 11)

add_executable(linux_sys_programming main.c
        sys_test/syscall_test/perror_test.h
        error_function.c
        get_num.h
        get_num.c
        sys_test/file_io_test/copy.c
        sys_test/file_io_test/fcntl_getfl_test.c
        sys_test/file_io_test/readv.c
        sys_test/process_test/mem_segement.c
        sys_test/process_test/display_env.c
        sys_test/process_test/long_jmp.c
        sys_test/process_test/set_jump_vars.c
        sys_test/mem_test/free_and_sbrk.c
        sys_test/user_group/check_passwd.c
        sys_test/sys_config_test/sys_conf.c
        sys_test/file_io_buf_test/direct_read.c
        sys_test/dirctory_and_link/unlink.c
        sys_test/dirctory_and_link/nftw_dir_tree.c
        sys_test/inotify/demo_inotify.c
        sys_test/signal/ouch.c
        sys_test/signal/int_quit.c
        sys_test/signal/sig_functions.h
        sys_test/signal/sig_functions.c
        sys_test/signal/sig_sender.c
        sys_test/signal/sig_receiver.c)
