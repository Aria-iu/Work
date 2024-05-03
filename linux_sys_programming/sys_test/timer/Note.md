## 间隔定时器
系统调用 setitimer()创建一个间隔式定时器（interval timer），这种定时器会在未来某个时
间点到期，并于此后（可选择地）每隔一段时间到期一次。
```c
#include <sys/time.h>
int settimer(int which,const struct itimerval *new_value,struct itimerval *old_value);
```
通过在调用 setitimer()时为 which 指定以下值，进程可以创建 3 种不同类型的定时器。
- ITIMER_REAL :创建以真实时间倒计时的定时器。到期时会产生 SIGALARM 信号并发送给进程。
- ITIMER_VIRTUAL :创建以进程虚拟时间（用户模式下的 CPU 时间）倒计时的定时器。到期时会产生信号
SIGVTALRM。
- ITIMER_PROF :创建一个 profiling 定时器，以进程时间（用户态与内核态 CPU 时间的总和）倒计时。到期时，则会产生 SIGPROF 信号。

可以在任何时刻调用 getitimer()，以了解定时器的当前状态、距离下次到期的剩余时间。
```c
#include <sys/time.h>
int gettimer(int which,struct itimerval* curr_value);
```
使用 setitimer()（和 alam()，稍后讨论）创建的定时器可以跨越 exec()调用而得以保存，
但由 fork()创建的子进程并不继承该定时器。

## 为阻塞操作设置超时
实时定时器的用途之一是为某个阻塞系统调用设置其处于阻塞状态的时间上限。例
如，当用户在一段时间内没有输入整行命令时，可能希望取消对终端的 read()操作。

## 暂停运行（休眠）一段固定时间
有时需要将进程挂起（固定的）一段时间。将前述定时器函数与 sigsuspend()相结合固然
可以达到这一目的，但使用休眠函数会更为简单

## POSIX时钟
POSIX 时钟（原定义于 POSIX.1b）所提供的时钟访问 API 可以支持纳秒级的时间精
度，其中表示纳秒级时间值的 timespec 结构同样也用于 nanosleep()

Linux 中，调用此 API 的程序必须以-lrt 选项进行编译，从而与 librt（realtime，实时）函
数库相链接。
POSIX 时钟 API 的主要系统调用包括获取时钟当前值的 clock_gettime()、返回时钟分辨率
的 clock_getres()，以及更新时钟的 clock_settime()。

```c
#define __POSIX_C_SOURCE 199309
#include <time.h>
int clock_gettime(clockid_t clockid,struct timespec *tp);
int clock_getres(clockid_t clockid,struct timespec *res);
int clock_settime(clockid_t clockid,const struct timespec *tp);

#define _XOPEN_SOURCE 600
int clock_getcpuclockid(pid_t pid,clockid_t *clockid);
#include <pthread.h>
int pthread_getcpuclockid(pthread_t thread,clockid_t *clockid);

```
clockid_t 是一种由 SUSv3 定义的数据类型，用于表示时钟标识符。

函数 clock_getcpuclockid()会将隶属于 pid 进程的 CPU 时间时钟的标识符置于 clockid 指针
所指向的缓冲区中。

函数 pthread_getcpuclockid()是 clock_getcpuclockid()的 POSIX 线程版，返回的标识符所标
识的时钟用于度量调用进程中指定线程消耗的 CPU 时间。

类似于 nanosleep()，Linux 特有的 clock_nanosleep()系统调用也可以暂停调用进程，直到
历经一段指定的时间间隔后，亦或是收到信号才恢复运行。
```c
#define _XOPEN_SOURCE 600
#include <time.h>
int clock_nanosleep(clockid_t clockid,int flags,
                    const struct timespec *request,struct timespec *remain);
```
POSIX 定时器 API 将定时器生命周期划分为如下几个阶段。
- 以系统调用 timer_create()创建一个新定时器，并定义其到期时对进程的通知方法。
- 以系统调用 timer_settime()来启动或停止一个定时器。
- 以系统调用 timer_delete()删除不再需要的定时器。
由fork()创建的子进程不会继承 POSIX 定时器。调用 exec()期间亦或进程终止时将停止并
删除定时器。
Linux 上，调用 POSIX 定时器 API 的程序编译时应使用-lrt 选项，从而与 librt（实时）函
数库相链接。

```c
#define __POSIX_C_SOURCE 199309
#include <time.h>
#include <signal.h>
int timer_create(clockid_t clock_id,struct sigevent *evp,timer_t *timerid);
int timer_settime(timer_t timerid,int flags,const struct itimerspec *value,
            struct itimerspec *old_spec);
int timer_gettime(timer_t timerid,struct itimerspec *curr_value);
int timer_delete(timer_t timerid);
```

## 利用文件描述符进行通知的定时器：timerfd API
始于版本 2.6.25，Linux 内核提供了另一种创建定时器的 API。Linux 特有的 timerfd API，可
从文件描述符中读取其所创建定时器的到期通知。因为可以使用 select()、poll()和 epoll()

timerfd_create()，它会创建一个新的定时器对象，并返回一个
指代该对象的文件描述符。

系统调用 timerfd_gettime()返回文件描述符 fd 所标识定时器的间隔及剩余时间。

系统调用 timerfd_settime()可以配备（启动）或解除（停止）由文件描述符 fd 所指代的定时器。
```c
#include <sys/timerfd.h>
int timerfd_create(int clockid,int flags);
int timerfd_settime(int fd,int flags,const struct itimerspec *new_value,
                    struct itimerspec *old_value);
int timerfd_gettime(int fd,struct itimerspec *curr_value);
```
调用 fork()期间，子进程会继承 timerfd_create()所创建文件描述符的拷贝。这些描述符与
父进程的对应描述符均指代相同的定时器对象，任一进程都可读取定时器的到期信息。
timerfd_create()创建的文件描述符能跨越 exec()得以保存（除非将描述符置为运行时关闭，
)，已配备的定时器在 exec()之后会继续生成到期通知。

一旦以 timerfd_settime()启动了定时器，就可以从相应文件描述符中调用 read()
来读取定时器的到期信息。出于这一目的，传给 read()的缓冲区必须足以容纳一个无符号 8 字节整型
（uint64_t）数。

程序若在后台暂停时定时器出现了多次到期。在程序恢复运行之后，第 1 次 read()调用就返回了所有这些到期。


