# SPI 硬件读写 W25Q64

这是一个使用 SPI 硬件读写 W25Q64 的示例。

{:02X}: 是一个格式化字符串，用于在 Rust 编程语言中将一个字节（u8 类型）转换为两位十六进制的字符串。例如，10 这个字节用{:02X}格式化后就是 0A，255 这个字节用{:02X}格式化后就是 FF。这里的 02 表示要用 0 来填充两位，如果不足两位的话。X 表示要用大写的字母来表示十六进制的数字，如果用 x 则表示用小写的字母。

## 执行指令

```shell
cargo rp spi_hard_w25q64
```

## 学习目标

- 了解 SPI 通讯协议
- 了解 W25Q64 非易失性存储器

## 接线图

![](../../../images/wiring_diagram/11-2%20硬件SPI读写W25Q64.jpg)
