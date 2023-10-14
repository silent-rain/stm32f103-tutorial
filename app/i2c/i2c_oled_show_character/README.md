# I2C OLED 通信协议显示字符

这是一个使用 I2C 通信协议在 OLED (有机发光二极管) 显示屏显示字符的示例。

## 执行指令

```shell
cargo rp i2c_oled_show_character
```

## 学习目标

- 了解 OLED 显示屏
  - 4 脚 OLED 一般使用 I2C
  - 7 脚 OLED 一般使用 SPI
  - 分辨率：`128*64`
- 配置 OLED 显示屏
- 了解调试工具
  - 串口调试
  - 显示屏调试
  - GDB 调试
  - Keil 调试
  - 其他
- 封装 OLED 字符发送函数

## 接线图

![](../../../images/wiring_diagram/4-1%20OLED显示屏.jpg)

## 相关文档

- [OLED 屏幕字库的建立](https://blog.csdn.net/weixin_44597885/article/details/129233163)
