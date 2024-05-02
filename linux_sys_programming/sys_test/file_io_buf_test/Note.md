##
系统 I/O 调用（即内核）和标准 C 语言库 I/O 函数（即 stdio 函数）在操作磁盘文件时会对数据进行缓冲

## 文件 I/O 的内核缓冲：缓冲区高速缓存
read()和 write()系统调用在操作磁盘文件时不会直接发起磁盘访问，而是仅仅在用户空间缓冲区与内核缓冲区高速缓存（kernel buffer cache）之间复制数据。

采用这一设计，意在使 read()和 write()调用的操作更为快速，因为它们不需要等待（缓慢
的）磁盘操作。同时，这一设计也极为高效，因为这减少了内核必须执行的磁盘传输次数。

## stdio 库的缓冲
当操作磁盘文件时，缓冲大块数据以减少系统调用，C 语言函数库的 I/O 函数（比如，
fprintf()、fscanf()、fgets()、fputs()、fputc()、fgetc()）正是这么做的。

调用 setvbuf()函数，可以控制 stdio 库使用缓冲的形式。
```c
#include<stdio.h>
int setvbuf(FILE* stream,char* buf,int mode,size_t size);
```
参数 buf 和 size 则针对参数 stream 要使用的缓冲区。

参数 mode 指定了缓冲类型，并具有下列值之一。
- _IONBF：不对 I/O 进行缓冲。每个 stdio 库函数将立即调用 write()或者 read()，并且忽略 buf 和 size
  参数，可以分别指定两个参数为 NULL 和 0。
- _IOLBF：采用行缓冲 I/O。指代终端设备的流默认属于这一类型。对于输出流，在输出一个换行符
  （除非缓冲区已经填满）前将缓冲数据。对于输入流，每次读取一行数据。
- _IOFBF：采用全缓冲 I/O。单次读、写数据（通过 read()或 write()系统调用）的大小与缓冲区相同。
  指代磁盘的流默认采用此模式。

## 控制文件 I/O 的内核缓冲

## 用于控制文件 I/O 内核缓冲的系统调用

```c
#include <unistd.h>
int fsync(int fd);
int fdatasync(int fd);
void sync(void);
```
fsync()系统调用将使缓冲数据和与打开文件描述符 fd 相关的所有元数据都刷新到磁盘
上。调用 fsync()会强制使文件处于 Synchronized I/O file integrity completion 状态。

fdatasync()系统 调 用 的运作 类 似 于 fsync() ，只是强制文件处于 synchronized I/O data
integrity completion 的状态。

sync()系统调用会使包含更新文件信息的所有内核缓冲区（即数据块、指针块、元数据等）刷新到磁盘上。

## 绕过缓冲区高速缓存：直接 I/O
始于内核 2.4，Linux 允许应用程序在执行磁盘 I/O 时绕过缓冲区高速缓存，从用户空间直
接将数据传递到文件或磁盘设备。有时也称此为直接 I/O（direct I/O）或者裸 I/O(raw I/O)。

可针对一个单独文件或块设备（比如，一块磁盘）执行直接 I/O。要做到这点，需要在调
用 open()打开文件或设备时指定 __O_DIRECT 标志(in linux)。

## 混合使用库函数和系统调用进行文件 I/O
在同一文件上执行 I/O 操作时，还可以将系统调用和标准 C 语言库函数混合使用。fileno()
和 fdopen()函数有助于完成这一工作。
```c
#include <stdio.h>
int fileno(FILE *stream);
FILE* fdopen(int fd,const char *mode);
```
给定一个（文件）流，fileno()函数将返回相应的文件描述符。随即可以在诸如 read()、write()、dup()和 fcntl()之类的 I/O 系统调用中正常
使用该文件描述符。

fdopen()函数与 fileno()函数的功能相反。给定一个文件描述符，该函数将创建了一个使用
该描述符进行文件 I/O 的相应流。



