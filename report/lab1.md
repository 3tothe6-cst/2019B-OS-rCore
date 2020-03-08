# Lab 1

## 回答一

首先保存通用寄存器和一些与中断有关的寄存器到一个位于内存中的 ``TrapFrame``，然后调用 ``rust_trap`` 函数访问该中断帧，最后恢复被保存的寄存器，重新执行造成中断的指令。

## 回答二

否。对于不可恢复的异常，程序将不能继续运行，那么中断处理程序这时就没有必要保存所有寄存器。

## 编程

查阅 ``Interrupt`` 枚举和 ``Exception`` 枚举的定义，猜测与非法指令异常有关的变体应是 ``Exception::IllegalInstruction``，据此在 ``rust_trap`` 函数的 match 语句中添加相应分支即可。具体做法见代码文件。

在 ``rust_main`` 函数中添加 ``mret`` 指令，并 ``make run``，可得到以下输出：

```
++++ setup interrupt! ++++
IllegalInstruction
panicked at 'IllegalInstruction', src/interrupt.rs:57:13
```

故上述做法是正确的。
