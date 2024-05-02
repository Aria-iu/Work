##
通过 Linux 专有的虚拟文件/proc/mounts，可查看当前已挂载文件系统的列表。
/proc/mounts 是内核数据结构的接口，因此总是包含已挂载文件系统的精确信息。

mount(8) 和 umount(8) 命 令 会 自 动 维 护 /etc/mtab 文 件 ， 该 文 件 所 包 含 的 信 息 与
/proc/mounts 的内容相类似，只是略微详细一些。特别是，etc/mtab 包含了传递给mount(8)的文件系统专有选项，
这并未在/proc/mounts 中出现。但是，因为系统调用
mount()和 umount()并不更新/etc/mtab，如果某些挂载或卸载了设备的应用程序没有更
新该文件，那么/etc/mtab 可能会变得不准确。

/etc/fstab（由系统管理员手工维护）包含了对系统支持的所有文件系统的描述，该文
件可供 mount(8)、umount(8)以及 fsck(8)所用。

## 挂载文件系统：mount()
mount()系统调用将由 source 指定设备所包含的文件系统，挂载到由 target 指定的目录下。
```c
#include <sys/mount.h>
int mount(const char *source,const char *target,const char *fstype
            unsigned long mountflags,const void *data);       
```
头两个参数分别命名为 source 和 target，其原因在于，除了将磁盘文件系统挂载到一目录
下之外，mount()还可以执行其他任务。
参数 fstype 是一字符串，用来标识设备所含文件系统的类型，比如，ext4 或 btrfs。
参数 mountflags 为一位掩码，通过对表 14-1 中所示的 0 个或多个标志进行或（OR）操作而得出。

## 卸载文件系统：umount()和 umount2()
umount()系统调用用于卸载已挂载的文件系统
```c
#include<sys/mount.h>
int umount(const char *target);
int umount2(const char *target,int flags);
```
target 参数指定待卸载文件系统的挂载点。

系统调用 umount2()是 umount()的扩展版。通过 flags 参数，umount2()可对卸载操作施以
更精密的控制。

## 获得与文件系统有关的信息：statvfs()
statvfs()和 fstatvfs()库函数能够获得与已挂载文件系统有关的信息。
```c
#include <sys/statvfs.h>
int statvfs(const char* pathname,struct statvfs *statvfsbuf);
int fstatvfs(int fd,struct statvfs *statvfsbuf);
```
statvfs()需使用 pathname 来指定文件系
统中任一文件的名称。而 fstatvfs()则需使用打开文件描述符 fd，来指代文件系统中的任一文
件。二者均返回一个 statvfs 结构，属于由 statvfsbuf 所指向的缓冲区，其中包含了关乎文件系
统的信息。


