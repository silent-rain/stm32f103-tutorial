# stm32f10x Rust 绑定

这是一个 C 语言的 stm32f10x 使用 Rust FFI 绑定的库。

## stm32f10x C 语言目录说明

- stm32f10x: C 程序目录;
- Library: 库函数文件;
- Start: 启动文件;
- System: 常用函数库;
- Conf: 配置包含关系和中断函数;

## 编译

绑定是动态生成。

```shell
cargo build --target thumbv7m-none-eabi --package stm32f10x_rs
```

## 测试

```shell
cargo test --target thumbv7m-none-eabi -p stm32f10x_rs
```
