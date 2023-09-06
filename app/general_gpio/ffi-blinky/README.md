# FFI 绑定版的闪烁 LED

这是一个 FFI 绑定版的闪烁 LED 的示例。

注意:

- 这里成功点亮 LED, 但是没有闪烁。
- 同时延时视乎存在异常。
- 毫秒与微秒错位了。

## 执行指令

```shell
cargo rp ffi-blinky

cargo run --target thumbv7m-none-eabi -p ffi-blinky probe-run -- --chip STM32F103C8 trace
```

## 逐步调制指令

```shell
# 编译
cargo build --target thumbv7m-none-eabi -p ffi-blinky

# 烧录
probe-run --chip STM32F103C8 target/thumbv7m-none-eabi/debug/ffi-blinky
```

## 学习目标

- 设置定时器
- 设置高电平
- 设置低电平

## 接线图

![](../../../images/wiring_diagram/3-1%20LED闪烁.jpg)
