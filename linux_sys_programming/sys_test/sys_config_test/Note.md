##
系统限制和选项会影响应用程序的行为，所以可移植应用程序需要获取限制值，弄
清系统对选项的支持情况。C 语言标准和 SUSv3 为此而提供了两种重要途径。
- 在编译程序时能够获得一些限制和选项。例如，int 类型的最大值取决于硬件结构和编
译器的设计选择。此类限制可在头文件中记录。
- 另外一些限制和选项在程序运行时可能会有变化。对此，SUSv3 定义了 3 个函数
sysconf()、pathconf()和 fpathconf()，供应用程序调用以检查系统实现的限制和选项。

在特定系统上获取限制，通常更为可取的方法是使用<limits.h>文件、sysconf()或 pathconf()。

## 在运行时获取系统限制（和选项）
sysconf()函数允许应用程序在运行时获得系统限制值。
```c
#include<unistd.h>
long sysconf(int name);
```
参数 name 应为定义于<unistd.h>文件中的_SC_系列常量之一

## 运行时获取与文件相关的限制（和选项）
pathconf()和 fpathconf()函数允许应用程序在运行时获取文件相关的限制值。
```c
#include<unistd.h>
long pathconf(const char *pathname, int name);
long fpathconf(int fd, int name);
```
pathconf()和 fpathconf()之间唯一的区别在于对文件或目录的指定方式。pathconf()采用路
径名方式来指定，而 fpathconf()则使用（之前已经打开的）文件描述符。

## 不确定的限制
有时，系统实现并未将一些系统限制定义为限制常量（比如：PATH_MAX），并且 sysconf()
或 pathconf()在返回相应限制(比如_PC_PATH_MAX)时会将其归为不确定。


