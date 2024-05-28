#include <stdio.h>
#include "P_total.h"
#include <archive.h>
#include <archive_entry.h>
#include <assert.h>

// gcc -o P_tar ./POSIX/P_tar.c -larchive
int main() {
    struct archive *a;
    struct archive_entry *entry;
    int ret;

    // 打开 tar 文件
    a = archive_read_new();
    // assert(a==NULL);
    archive_read_support_format_tar(a);
    ret = archive_read_open_filename(a, "example.tar", 10240); // 替换为你的 tar 文件名
    if (ret != ARCHIVE_OK) {
        fprintf(stderr, "Error opening tar file\n");
        return 1;
    }

    // 读取 tar 文件中的每个文件信息
    while (archive_read_next_header(a, &entry) == ARCHIVE_OK) {
        printf("File name: %s, Size: %jd bytes\n", archive_entry_pathname(entry),
               (intmax_t)archive_entry_size(entry));
    }

    // 关闭 tar 文件
    archive_read_close(a);
    archive_read_free(a);
    return 0;
}