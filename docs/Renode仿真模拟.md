# Renode 仿真模拟

## 仿真配置文件

```shell
# @scripts/stm32f103.resc
cat stm32f103.resc
```

## 修改默认启动仿真程序

```shell
# 修改该行目标程序位置
$bin=@target/thumbv7m-none-eabi/debug/hello
```

## 启动 Renode CLI

打开终端，切换至项目根目录;
启动 Renode CLI 后会打开一个新的终端窗口;

```shell
renode
```

## 启动 STM32 仿真器

```shell
start @scripts/stm32f103.resc
```

## Renode 在 GDB 连接后立即启动整个模拟。

```shell
machine StartGdbServer 3333 true
```

## GDB 连接 Renode

```shell
# 启动 GDB
# 默认启动
# arm-none-eabi-gdb target/thumbv7m-none-eabi/debug/hello
# 指定当前目录下的.gdbinit配置文件启动
arm-none-eabi-gdb -iex 'add-auto-load-safe-path .' -q target/thumbv7m-none-eabi/debug/blinky

# 连接 Renode
target remote :3333
```

## 重启仿真器

```shell
machine Reset
```

## 清除仿真程序

```shel
Clear
```

## 查看所有外设

```shell
peripherals
```

## 系统总线挂钩

- 在访问特定外设进行读取后执行 Python 脚本

```shell
(machine) sysbus SetHookAfterPeripheralRead gpioPortA "print '%s peripheral has been accessed to read'"
```

- 在访问特定外围设备进行写入之前执行 Python 脚本

```shell
(machine) sysbus SetHookBeforePeripheralWrite peripheral "print '%s peripheral has been accessed to write'"

```

## 参考文档

[renode 指南](https://renode.readthedocs.io/en/latest/introduction/installing.html)
[renode 代码仓库](https://github.com/renode/renode)
[在 Renode 中使用 Python](https://renode.readthedocs.io/en/latest/basic/using-python.html)
[Renode 开启 GDB 调试](https://renode.readthedocs.io/en/latest/debugging/gdb.html)
[使用 GDB 进行调](https://jzow.github.io/discovery/microbit/05-led-roulette/debug-it.html)
