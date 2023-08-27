# 旋转编码器计次

这是一个使用旋转编码器次计次的示例。正向旋转输出正数，负向旋转输出负数。

## 学习目标

- 了解接入编码器
  - 若编码器与教程不一致, 可自行按引脚进行接入
- 建议：
  - 在中断函数中最好不要执行耗时的代码，中断是处理突发事件的不适合耗时操作；
  - 不要在主程序和中断程序中操作可能产生冲突的硬件，可以用变量或标志位进行处理；

## 接线图

![](../../images/5-2%20旋转编码器计次.jpg)