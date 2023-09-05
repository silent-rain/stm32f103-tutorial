# FFI Hello

这是一个使用 Rust FFI 绑定 C 语言的案例。

## 安装 ARM GCC 编译环境

```shell
sudo pacman -S arm-none-eabi-gcc arm-none-eabi-newlib
```

## 编译

绑定是动态生成。

```shell
cargo build --package ffi_hello
```

## 测试

```shell
cargo test --target thumbv7m-none-eabi -p ffi_hello
```
