# FFI 绑定版的闪烁 LED

这是一个 FFI 绑定版的闪烁 LED 的示例。

注意: 这是一次失败的尝试。

## 执行指令

```shell
cargo rp ffi-blinky

cargo run --target thumbv7m-none-eabi -p ffi-blinky probe-run -- --chip STM32F103C8 trace
```

## 学习目标

- 设置定时器
- 设置高电平
- 设置低电平

## 接线图

![](../../images/../../images/3-1%20LED闪烁.jpg)
