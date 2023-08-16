# Linux st-link 配置

## st-link 与开发板连线

- 注意连线顺序可能与教程不一样；

## udev

```shell
git clone https://github.com/stlink-org/stlink

cd stlink
sudo cp config/udev/rules.d/49-stlinkv2-1.rules /etc/udev/rules.d

# 重新加载udev规则
sudo udevadm control --reload-rules
```

## 查看设备是否加载

```shell
$ lsusb |grep 'ST-LINK/V2'

Bus 003 Device 033: ID 0483:3748 STMicroelectronics ST-LINK/V2
```

## 相关文档

- [Linux 系统如何安装 ST-Link v2 烧录环境](https://www.yisu.com/zixun/501716.html)
- [linux 下 st-link 开发 STM32](https://codeantenna.com/a/gATpTrEEwz)
- [安装 ST-Link GDBServer](https://blog.51cto.com/zoomdy/5871707)
- [ST-link 驱动下载、安装、配置和升级](https://blog.csdn.net/qq_52158753/article/details/130161426)
