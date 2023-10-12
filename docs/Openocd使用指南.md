# Openocd 使用指南

## 用户配置文件

linux 中配置文件一般所在目录 `/usr/share/openocd/scripts`

三种主要类型的非用户配置文件:

- interface: 接口配置文件;
- board: 特定于电路板的，它设置 JTAG TAPS 和 他们的 GDB 目标（通过推迟到某个文件），声明所有闪存;;
- target: 集成 CPU 和其他 JTAG TAPS 的芯片

适合当前开发板的配置文件：

- board: /usr/share/openocd/scripts/board/stm32f103c8_blue_pill.cfg
- target: /usr/share/openocd/scripts/target/stm32f1x.cfg

## 配置

```text
# 使用CMSIS-DAP协议进行SWD调试
# source [find interface/cmsis-dap.cfg]

# stlink
source [find interface/stlink.cfg]
# source [find interface/stlink-v2.cfg]


# 目标设备芯片类型和调试速率
set TARGET_NAME "cortex_m"
set CHIPNAME "stm32f103c8"

# clock speed 1000 kHz
# adapter speed 1000

# 目标设备连接方式和接口编号
transport select hla_swd
# set CONNECT_MODE smp
# set CONNECT_TYPE hla_swd
# hla_layout stlink
# hla_port 4242

# 选择目标设备
source [find target/stm32f1x.cfg]
```

## 连接目标设备

```shell
openocd -f openocd.cfg
```

## 参考文档

- [openocd 调试使用指南](https://www.python100.com/html/5F3U4P5L64PA.html)
- [STM32F3DISCOVERY openocd 调试](https://xxchang.github.io/book/start/hardware.html)
