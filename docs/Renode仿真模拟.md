# Renode 仿真模拟

## 仿真配置文件

```shell
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
start @stm32f103.resc
```


## Renode 在 GDB 连接后立即启动整个模拟。

```shell
machine StartGdbServer 3333 true
```

runMacro sysbus LoadELF @target/thumbv7m-none-eabi/debug/hello


## GDB 连接 Renode

```shell
# 启动 GDB
arm-none-eabi-gdb target/thumbv7m-none-eabi/debug/hello

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


## 参考文档
[Renode 开启 GDB 调试](https://renode.readthedocs.io/en/latest/debugging/gdb.html)
[使用 GDB 进行调](https://jzow.github.io/discovery/microbit/05-led-roulette/debug-it.html)