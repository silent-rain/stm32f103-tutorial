# PWM 驱动呼吸灯 2

这是一个使用 PWM 驱动多个呼吸灯的示例。这里将介绍 Channel、断点、GDB 的综合使用。

这里例子来自依赖库的官方案例；

## 执行指令

```shell
cargo rp pwm_led2
```

## 学习目标

- 了解 Channel 用法
- 了解 bkpt 的用法
- 结合 GDB 进行电灯

## 接线图

![](../../../images/wiring_diagram/6-3%20PWM驱动LED呼吸灯.jpg)
