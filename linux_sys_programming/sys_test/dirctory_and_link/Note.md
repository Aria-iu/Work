## 创建和移除（硬）链接：link()和 unlink()
link()和 unlink()系统调用分别创建和移除硬链接。
```c
#include <unistd.h>
int link(const char *oldpath,const char *newpath);
int unlink(const char *pathname);
```
## 使用符号链接：symlink()和 readlink()
symlink()系统调用会针对由 filepath 所指定的路径名创建一个新的符号链接—linkpath。
（想移除符号链接，需使用 unlink()调用。）
```c
#include <unistd.h>
int symlink(const char *filepath,const char *linkpath);
```
如果指定一符号链接作为 open()调用的 pathname 参数，那么将打开链接指向的文件。有
时，倒宁愿获取链接本身的内容，即其所指向的路径名。这正是 readlink()系统调用的本职工
作，将符号链接字符串的一份副本置于 buffer 指向的字符数组中。
```c
#include <unistd.h>
int readlink(const char *pathname,char *buffer,size_t bufsize);
```

## 读目录：opendir()和 readdir()
opendir()函数打开一个目录，并返回指向该目录的句柄，供后续调用使用。
除了要创建的目录流所针对的目录由打开文件描述符指代之外，fdopendir()与 opendir()
并无不同。

readdir()函数从一个目录流中读取连续的条目。

rewinddir()函数可将目录流回移到起点，以便对 readdir()的下一次调用将从目录的第一个
文件开始。

closedir()函数将由 dirp 指代、处于打开状态的目录流关闭，同时释放流所使用的资源。

有一个目录流，就有一个文件描述符与之关联。dirfd()函数返回与 dirp 目录流相关联的文
件描述符。

## 文件树遍历：nftw()
nftw()函数允许程序对整个目录子树进行递归遍历，并为子树中的每个文件执行某些操作

nftw()函数遍历由 dirpath 指定的目录树，并为目录树中的每个文件调用一次由程序员定义
的 func 函数。

```c
#define _XOPEN_SOURCE 500
#include <ftw.h>
int nftw(const char *dirpath,
         (int)(*func)(const char *pathname,const struct stat *statbuf,
                        int typeflag,struct FTW *ftwbuf),
         int nopenfd,int flags);
```

## 改变进程的根目录：chroot()
每个进程都有一个根目录，该目录是解释绝对路径（即那些以/开始的目录）时的起点。
默认情况下，这是文件系统的真实根目录。（新进程从其父进程处继承根目录。）有些场合需
要改变一个进程的根目录，而特权级（CAP_SYS_CHROOT）进程通过 chroot()系统调用能够
做到这一点。

```c
#include <unistd.h>
int chroot(const char *pathname);
```
chroot()系统调用将进程的根目录改为由 pathname 指定的目录。

**ftp 程序就是应用 chroot()的典型实例之一。作为一种安全措施，当用户匿名登录 ftp 时，ftp
程序将使用 chroot()为新进程设置根目录—一个专门预留给匿名登录用户的目录。调用
chroot()后，用户将受困于文件系统中新根目录下的子树中，无法在整个文件系统中信马由缰。**

