# 独立看门狗

这是一个独立看门狗的示例。独立工作，对时间精度要求较低。

程序正常运行时，第二行显示 RST;
按住按键 5s 不放模拟程序卡死，看门狗触发复位, 第二行显示 IWDGRST。

## 执行指令

```shell
cargo rp iwdg
```

## 学习目标

- 了解独立看门狗
- 超时时间：0.1ms ～ 26214.4ms

## 接线图

![](../../../images/wiring_diagram/14-1%20独立看门狗.jpg)
