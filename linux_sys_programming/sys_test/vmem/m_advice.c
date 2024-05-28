
#ifdef __linux__
#define _BSD_SOURCE
#endif
#include <sys/mman.h>
#include <sys/stat.h>
#include <fcntl.h>
#include "../total.h"
#include <unistd.h>

#define MAP_SIZE 4096		// 4K 1个页
#define WRITE_SIZE 10

int
main(int argc, char *argv[])
{
    char *addr;
    int fd, j;

    setbuf(stdout, NULL);

    if (argc != 2 || strcmp(argv[1], "--help") == 0)
        usageErr("%s file\n", argv[0]);

    unlink(argv[1]);
    fd = open(argv[1], O_RDWR | O_CREAT, S_IRUSR | S_IWUSR);
    if (fd == -1)
        errExit("open");

    for (j = 0; j < MAP_SIZE; j++)
        write(fd, "a", 1);
    if (fsync(fd) == -1)
        errExit("fsync");
    close(fd);

    fd = open(argv[1], O_RDWR);
    if (fd == -1)
        errExit("open");

    addr = mmap(NULL, MAP_SIZE, PROT_READ | PROT_WRITE, MAP_PRIVATE, fd, 0);
    if (addr == MAP_FAILED)
        errExit("mmap");

    printf("After mmap:          ");
    write(STDOUT_FILENO, addr, WRITE_SIZE);		// aaaaaaaaaa
    printf("\n");

    /* Copy-on-write semantics mean that the following modification
       will create private copies of the pages for this process */

    for (j = 0; j < MAP_SIZE; j++)
        addr[j]++;

    printf("After modification:  ");
    write(STDOUT_FILENO, addr, WRITE_SIZE);		// bbbbbbbbbbb
    printf("\n");

    /* After the following, the mapping contents revert to the original file
       contents (if MADV_DONTNEED has destructive semantics, as on Linux) */
    /*
     * 调用进程不再要求这个区域中的分页驻留在内存中。
     * 对于 MAP_PRIVATE 区域来讲，
     * 映射分页会显式地被丢弃，这意味着所有发生在分页上的变更会丢失。
     * 虚拟内存地址范围仍然可
     * 访问，但对各个分页的下一个访问将会导致一个分页故障和分页的重新初始化，这种初始化要
     * 么使用其映射的文件内容，要么在匿名映射的情况下就使用零来初始化。(此处是非匿名映射)
     * */
    if (madvise(addr, MAP_SIZE, MADV_DONTNEED) == -1)
        errExit("madvise");

    printf("After MADV_DONTNEED: ");
    write(STDOUT_FILENO, addr, WRITE_SIZE);        // aaaaaaaaaa
    printf("\n");

    exit(EXIT_SUCCESS);
}