# Lab 6

采用与 ``RRScheduler`` 类似的实现思路，在 ``StrideScheduler`` 中也包含一个 ``threads`` 域和一个 ``current`` 域，同时 ``push()`` 和 ``exit()`` 的实现直接仿造了 ``RRScheduler``。在 ``pop()`` 的实现中，找出各 ``StridePassInfo`` 中的 ``stride`` 域最小者，然后将其取出；在 ``tick()`` 的实现中，令当前线程 ``stride`` = ``stride`` + ``pass``，并返回 true，代表当前线程应立即被切换出去。

对于 ``StridePassInfo``，令 ``stride`` 的初值为零，``pass`` 的初值为 65536。另外，``sys_setpriority()`` 被实现为令当前线程的 ``pass`` 为 65536/i，其中 i 为优先级。

最终的一个测试输出：

```
thread 0 exited, exit code = 0
thread 5 exited, exit code = 292400
thread 4 exited, exit code = 230000
thread 3 exited, exit code = 174000
thread 2 exited, exit code = 116400
thread 1 exited, exit code = 59200
```
