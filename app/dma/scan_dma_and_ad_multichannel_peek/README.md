# DMA+AD 多通道分批读取

这是一个 DMA+AD 多通道的示例。使用 ADC 的扫描模式加 DMA 数据转运。以阻塞的方式读取数据。ADC 接口循环 DMA RX 传输测试

## 执行指令

```shell
cargo rp scan_dma_and_ad_multichannel_peek
```

## 学习目标

- 了解 DMA

## 接线图

![](../../../images/wiring_diagram/8-2%20DMA+AD多通道.jpg)
