# 环境搭建

## 系统环境

- 系统版本: Archlinux
- Rust 版本: 1.71.0
- Cargo 版本: 1.71.0
- Rustup 版本: 1.26.0

## 安装工具链

工具链简介：

- thumbv6m-none-eabi，适用于 Cortex-M0 和 Cortex-M1 处理器
- thumbv7m-none-eabi，适用于 Cortex-M3 处理器
- thumbv7em-none-eabi，适用于 Cortex-M4 和 Cortex-M 处理器
- thumbv7em-none-eabihf，适用于 Cortex-M4F 和 Cortex-M7F 处理器
- thumbv8m.main-none-eabi，适用于 Cortex-M33 和 Cortex-M35P 处理器
- thumbv8m.main-none-eabihf，适用于 Cortex-M33F 和 Cortex-M35PF 处理器

```shell
rustup target add thumbv7m-none-eabi
```

## 安装 cargo-binutils

```shell
rustup component add llvm-tools-preview

# 安装指定版本
# cargo install cargo-binutils --vers 0.3.3
# 安装最新版本
cargo install cargo-binutils

cargo size --version
```

## 安装 cargo-embed

```shell
cargo install cargo-embed

cargo embed --version
```

## 安装 ARM GCC 编译环境

```shell
sudo pacman -S arm-none-eabi-gcc arm-none-eabi-newlib
```

## 有 ARM 支持的 GDB

有 ARM 仿真支持的 QEMU 用于调试 ARM Cortex-M 程序的 GDB 命令

```shell
sudo pacman -S arm-none-eabi-gdb
```

## 终端串口工具

```shell
sudo pacman -S minicom
```

## 图形化串口工具（可选）

```shell
yay -S aur/cutecom
```

## Openocd 调试器

```shell
sudo pacman -S openocd
```

## 仿真模拟器

### renode 模拟器(推荐)

#### 安装

```
wget https://github.com/renode/renode/releases/download/v1.14.0/renode-1.14.0-1-x86_64.pkg.tar.xz

sudo pacman -U ./renode-1.14.0-1-x86_64.pkg.tar.xz

renode -v
```

### 有 ARM 仿真支持的 QEMU

使用该模拟器需要安装 stm32 的插件

#### 安装

- 默认不支持 stm32

```shell
sudo pacman -S qemu-system-arm
```

#### 运行

这个程序将会阻塞住。

```shell
qemu-system-arm \
      -cpu cortex-m3 \
      -machine lm3s6965evb \
      -gdb tcp::3333 \
      -S \
      -nographic \
      -kernel target/thumbv7m-none-eabi/debug/blinky
```
