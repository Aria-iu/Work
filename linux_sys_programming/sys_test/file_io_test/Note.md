# file io
所有执行 I/O 操作的系统调用都以文件描述符，一个非负整数（通常是小整数），来指代
打开的文件。文件描述符用以表示所有类型的已打开文件，包括管道（pipe） 、FIFO、socket、终端、
设备和普通文件。针对每个进程，文件描述符都自成一套。

## funcs
- fd = open(pathname, flags, mode) 函数打开 pathname 所标识的文件，并返回文件描
述符，用以在后续函数调用中指代打开的文件。如果文件不存在，open()函数可以
创建之，这取决于对位掩码参数 flags 的设置。flags 参数还可指定文件的打开方式：只
读、只写亦或是读写方式。mode 参数则指定了由 open()调用创建文件的访问权限，
如果 open()函数并未创建文件，那么可以忽略或省略 mode 参数。
- numread = read(fd, buffer, count) 调用从 fd 所指代的打开文件中读取至多 count 字节的
数据，并存储到 buffer 中。read()调用的返回值为实际读取到的字节数。如果再无字节
可读（例如：读到文件结尾符 EOF 时），则返回值为 0。
- numwritten = write(fd, buffer, count) 调用从 buffer 中读取多达 count 字节的数据写入由
fd 所指代的已打开文件中。write()调用的返回值为实际写入文件中的字节数，且有可
能小于 count。
- status = close(fd)在所有输入/输出操作完成后，调用 close()，释放文件描述符 fd 以及
与之相关的内核资源

## **通用性**
UNIX I/O 模型的显著特点之一是其输入/输出的通用性概念。这意味着使用 4 个同样的系
统调用 open()、read()、write()和 close()可以对所有类型的文件执行 I/O 操作，包括终端之类的
设备。因此，仅使用这些系统调用编写的程序，将对任何类型的文件有效。

例如，/sys_test/file_io_test/copy.c
./copy test test.old
./copy a.txt /dev/tty   输出到终端

要实现通用 I/O， 就必须确保每一文件系统和设备驱动程序都实现了相同的 I/O 系统调用集。

文件系统或设备所特有的操作细节在内核中处理，在编程时通常可以忽略设备专有的因
素。一旦应用程序需要访问文件系统或设备的专有功能时，可以选择瑞士军刀般的 ioctl()系统
调用

### open(pathname, flags mode)
#### flags 参数
open()系统调用的 flags 参数值介绍

标 志                 用 途                             统一 UNIX 规范版本
O_RDONLY            以只读方式打开                             v3
O_WRONLY            以只写方式打开                             v3
O_RDWR              以读写方式打开                             v3
O_CLOEXEC           设置 close-on-exec 标志（自 Linux 2.6.23 版本开始）v4
O_CREAT             若文件不存在则创建之                          v3
O_DIRECT    无缓冲的输入/输出O_DIRECTORY如果 pathname 不是目录，则失败    v4
O_EXCL           结合 O_CREAT 参数使用，专门用于创建文件           v3
O_LARGEFILE在 32 位系统中使用此标志打开大文件O_NOATIME调用 read()时，不修改文件最近访问时间（自 Linux 2.6.8
版本开始）
O_NOCTTY         不要让 pathname（所指向的终端设备）成为控制终端    v3
O_NOFOLLOW          对符号链接不予解引用                          v4
O_TRUNC             截断已有文件，使其长度为零                   v3
O_APPEND            总在文件尾部追加数据                          v3
O_ASYNC             当 I/O 操作可行时，产生信号（signal）通知进程   
O_DSYNC         提供同步的 I/O 数据完整性（自 Linux 2.6.33 版本开始）v3
O_NONBLOCK          以非阻塞方式打开                            v3
O_SYNC              以同步方式写入文件                           v3

#### open()的错误返回
若打开文件时发生错误，open()将返回−1，错误号 errno 标识错误原因。

- EACCES：文件权限不允许调用进程以 flags 参数指定的方式打开文件。无法访问文件，其可能的原
  因有目录权限的限制、文件不存在并且也无法创建该文件。
- EISDIR：所指定的文件属于目录，而调用者企图打开该文件进行写操作。不允许这种用法。
- EMFILE：进程已打开的文件描述符数量达到了进程资源限制所设定的上限
- ENFILE：文件打开数量已经达到系统允许的上限
- ENOENT：要么文件不存在且未指定 O_CREAT 标志，要么指定了 O_CREAT 标志，但 pathname 参数所指定路径的目录之一不存在，或者 pathname 参数为符号链接，而该链接指向的文件不存
  在（空链接）。
- EROFS：所指定的文件隶属于只读文件系统，而调用者企图以写方式打开文件。
- ETXTBSY：所指定的文件为可执行文件（程序），且正在运行。系统不允许修改正在运行的程序。

### read()
### write()
### close()
### lseek()
### ioctl()
in sys/ioctl.h

int ioctl(int fd,int request, ... );

fd 参数为某个设备或文件已打开的文件描述符，request 参数指定了将在 fd 上执行的控制操
作。具体设备的头文件定义了可传递给 request 参数的常量。

ioctl()调用的第三个参数采用了标准 C 语言的省略符号（...）来表示（称之为 argp），可以
是任意数据类型。ioctl()根据 request 的参数值来确定 argp 所期望的类型。通常情况下，argp
是指向整数或结构的指针，有些情况下，不需要使用 argp。

# file ops

fcntl()系统调用对一个打开的文件描述符执行一系列控制操作。

in fcntl.h

int fcntl(int fd, int cmd, ... );

cmd 参数所支持的操作范围很广。fcntl()的第三个参数以省略号来表示，这意味着可以将其设置为不同的类型，或者加以省
略。内核会依据 cmd 参数（如果有的话）的值来确定该参数的数据类型。

## 打开文件的状态标志

fcntl()的用途之一是针对一个打开的文件，获取或修改其访问模式和状态标志（这些值是
通过指定 open()调用的 flag 参数来设置的）。要获取这些设置，应将 fcntl()的 cmd 参数设置为
F_GETFL。

### dup(2) dup() dup(3) fcntl(oldfd,F_DUPFD,startfd)

### pread() pwrite()
pread()调用等同于将lseek()和read()纳入同一原子操作

### readv() 和 writev()
readv()和 writev()系统调用分别实现了分散输入和集中输出的功能。

readv()调用和 writev()调用的主要优势在于便捷。如下两种方案，任选其一都可替代对
writev()的调用。
- 编码时，首先分配一个大缓冲区，随即再从进程地址空间的其他位置将数据复制过来，
最后调用 write()输出其中的所有数据。
- 发起一系列 write()调用，逐一输出每个缓冲区中的数据。

```c
in sys/uio.h

ssize_t readv( int fd, const struct iovec *iov, int iovcnt);
ssize_t writev( int fd, const struct iovec *iov, int iovcnt);
```
这些系统调用并非只对单个缓冲区进行读写操作，而是一次即可传输多个缓冲区的数据。
数组 iov 定义了一组用来传输数据的缓冲区。整型数 iovcnt 则指定了 iov 的成员个数。iov 中的
每个成员都是如下形式的数据结构。

SUSv3 标准允许系统实现对 iov 中的成员个数加以限制。系统实现可以通过定义
<limits.h>文件中 IOV_MAX 来通告这一限额，程序也可以在系统运行时调用 sysconf (_SC_
IOV_MAX)来获取这一限额。
```c
struct iovec{
  void *iov_base;
  size_t iov_len;
}
```
#### 分散输入

readv()系统调用实现了分散输入的功能：从文件描述符 fd 所指代的文件中读取一片连续
的字节，然后将其散置（“分散放置”）于 iov 指定的缓冲区中。这一散置动作从 iov[0]开始，
依次填满每个缓冲区。原子性是 readv()的重要属性。返回读取的字节数。

#### 集中输出

writev()系统调用实现了集中输出：将 iov 所指定的所有缓冲区中的数据拼接（ “集中” ）起
来，然后以连续的字节序列写入文件描述符 fd 指代的文件中。对缓冲区中数据的“集中”始于
iov[0]所指定的缓冲区，并按数组顺序展开。

writev()调用也属于原子操作，即所有数据将一次性地从用户内存传输到 fd 指代的文件中。

#### 在指定的文件偏移量处执行分散输入/集中输出

preadv()、pwritev()，将分散输入/集中输出和于指 定文件偏移量处的 I/O 二者集于一身。
```c
in sys/uio.h

ssize_t preadv( int fd, const struct iovec *iov, int iovcnt,off_t offset);
ssize_t pwritev( int fd, const struct iovec *iov, int iovcnt,off_t offset);
```
preadv()和 pwritev()系统调用所执行的任务与 readv()和 writev()相同，但执行 I/O 的位置将
由 offset 参数指定（类似于 pread()和 pwrite()系统调用） 。

对于那些既想从分散-集中 I/O 中受益，又不愿受制于当前文件偏移量的应用程序（比如，
多线程的应用程序）而言，这些系统调用恰好可以派上用场。

### 截断文件：truncate()和 ftruncate()系统调用
```c
in unistd.h

int truncate( const char *pathname, off_t length);
int ftruncate( int fd, off_t length);
```
若文件当前长度大于参数 length，调用将丢弃超出部分，若小于参数 length，调用将在文
件尾部添加一系列空字节或是一个文件空洞。

### 非阻塞 I/O
在打开文件时指定 O_NONBLOCK 标志，目的有二。
- 若 open()调用未能立即打开文件，则返回错误，而非陷入阻塞。有一种情况属于例外，
调用 open()操作 FIFO 可能会陷入阻塞
- 调用 open()成功后，后续的 I/O 操作也是非阻塞的。若 I/O 系统调用未能立即完成，则可
  能会只传输部分数据，或者系统调用失败，并返回 EAGAIN 或 EWOULDBLOCK 错
  误。具体返回何种错误将依赖于系统调用。Linux 系统与许多 UNIX 实现一样，将两
  个错误常量视为同义。

管道、FIFO、套接字、设备（比如终端、伪终端）都支持非阻塞模式

### 大文件 I/O
### /dev/fd目录
### 创建临时文件
mkstemp()和 tmpfile()。

mkstemp()函数生成一个唯一文件名并打开该文件，返回一个可用于 I/O 调用的文件描述符。
```c
#include <stdlib.h>
int mkstemp(char *template);

#include <stdio.h>
FILE *tmpfile(void);
```
模板参数采用路径名形式，其中最后 6 个字符必须为 XXXXXX。这 6 个字符将被替换，
以保证文件名的唯一性，且修改后的字符串将通过 template 参数传回。因为会对传入的 template
参数进行修改，所以必须将其指定为字符数组，而非字符串常量。

tmpfile()函数会创建一个名称唯一的临时文件，并以读写方式将其打开。

tmpfile()函数执行成功，将返回一个文件流供 stdio 库函数使用。文件流关闭后将自动删
除临时文件。为达到这一目的，tmpfile()函数会在打开文件后，从内部立即调用 unlink()来删
除该文件名。

