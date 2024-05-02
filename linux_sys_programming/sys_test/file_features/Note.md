## 获取文件信息：stat()
利用系统调用 stat()、lstat()以及 fstat()，可获取与文件有关的信息，其中大部分提取自文
件 i 节点。
```c
#include<sys/stat.h>
int stat(const char *pathname,struct stat *statbuf);
int lstat(const char *pathname,struct stat *statbuf);
int fstat(int fd ,struct stat *statbuf);
```
以上 3 个系统调用之间仅有的区别在于对文件的描述方式不同。

系 统 调 用 stat() 和 lstat() 无 需 对 其 所 操 作 的 文 件 本 身 拥 有 任 何 权 限 ， 但 针 对 指 定
pathname 的父目录要有执行（搜索）权限。而只要供之以有效的文件描述符，fstat()系统调用
总是成功。

## 文件时间戳
stat 结构的 st_atime、st_mtime 和 st_ctime 字段所含为文件时间戳，分别记录了对文件的
上次访问时间、上次修改时间，以及文件状态（即文件 i 节点内信息）上次发生变更的时间。
对时间戳的记录形式为自 1970 年 1 月 1 日（参见 10.1 节）以来所历经的秒数。

### 使用 utime()和 utimes()来改变文件时间戳
```c
#include <utime.h>
int utime(const char *pathname,const struct utimbuf *buf);
```
参数 pathname 用来标识欲修改时间的文件。若该参数为符号链接，则会进一步解除引用。
参数 buf 既可为 NULL，也可为指向 utimbuf 结构的指针。

### 使用 utimensat()和 futimens()改变文件时间戳
utimensat()系统调用（内核自 2.6.22 版本开始支持）和 futimens()库函数（glibc 自版本 2.6
开始支持）为设置对文件的上次访问和修改时间戳提供了扩展功能。

```c
#include <sys/stat.h>
int utimesat(int dirfd,const char *pathname,const struct timespec times[2],int flags);
```
若将 times 指定为 NULL，则会将以上两个文件时间戳都更新为当前时间。若 times 值为
非 NULL，则会针对指定文件在 times[0]中放置新的上次访问时间，在 times[1]中放置新的上
次修改时间。

## 文件属主
每个文件都有一个与之关联的用户 ID（UID）和组 ID（GID），籍此可以判定文件的属主
和属组。

## 文件权限


