# PWM 驱动呼吸灯-端口重映射

这是一个使用 PWM 驱动多个呼吸灯的示例。这里将介绍 Channel、断点、GDB 的综合使用。

## 执行指令

```shell
cargo rp pwm_led_remap
```

## 学习目标

- 了解端口重映射方式
- PA0 -> PA15, 同时注意进行调试功能

## 接线图

![](../../../images/wiring_diagram/6-3%20PWM驱动LED呼吸灯.jpg)
