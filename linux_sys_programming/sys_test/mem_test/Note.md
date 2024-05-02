## 调整 program break：brk()和 sbrk()
改变堆的大小（即分配或释放内存），其实就像命令内核改变进程的 program break 位置一样简单。

```c
#include <unistd.h>
int brk(void *end_data_segement);
void *sbrk(intptr_t increment);
```
系统调用 brk()会将 program break 设置为参数 end_data_segment 所指定的位置。内存以页为单位进行分配，end_data_segment 实际会四舍五入到下一个内存页的边界处。

