# STM32F103 开发板使用案例

这是一个关于 STM32F103 开发板使用案例，记录一些使用 Rust 语言进行开发测试的案例。

该教程将主要根据 [B 站江科大](https://www.bilibili.com/video/BV1th411z7sn?p=1&vd_source=c459b4f4f90bc42bb5ddb5baf12e1bc7) 的视频教学进行学习嵌入式开发。示例中将会借用 `B 站江科大` 视频配套资料中的一些资源进行展示。

同时也会借用 `stm32f1xx-hal` 开发板库中的示例，进行整合在一起学习。

## 示例目录

### 基础示例

- [Hello World](./app/basic/helloworld)
- [自定义异常 Panic](./app/basic/panics)
- [烧录工具 Embed](./app/basic/flash_tool_embed)
- [烧录工具 probe-run](./app/basic/flash_tool_defmt)
- [单元测试套件](./app/basic/unit_testsuite)

### 延迟

- [系统计时器延迟](./app/delay/syst_timer_delay)
- [系统定时器延迟](./app/delay/syst_delay)
- [TIM2 定时器延迟](./app/delay/tim2_delay)
- [汇编延迟](./app/delay/asm_delay)

### 通用 GPIO

- [点灯](./app/general_gpio/turns_user_led)
- [闪烁 LED](./app/general_gpio/blinky)
- [运行中更改 GPIO 模式](./app/general_gpio/multi_mode_gpio)
- [动态设置 GPIO 模式](./app/general_gpio/dynamic_gpio)
- [闪烁 LED](./app/general_gpio/blinky)
- [计时器闪烁 LED](./app/general_gpio/timer_blinky)
- [延迟闪烁 LED](./app/general_gpio/delay_blinky)
- [TIM2 通用定时器延迟闪烁 LED](./app/general_gpio/tim2_timer_delay_blinky)
- [LED 流水灯](./app/general_gpio/led_flow_light)
- [蜂鸣器](./app/general_gpio/buzzer)
- [按键控制 LED](./app/general_gpio/key_control_led)
- [光敏传感器控制蜂鸣器](./app/general_gpio/light_sensor_control_buzzer)
- [FFI 绑定版的闪烁 LED](./app/general_gpio/ffi-blinky)

### 中断

- [对射式红外传感器计次](./app/interrupt/opposing_infrared_sensor_count)
- [对射式红外传感器计次 2](./app/interrupt/opposing_infrared_sensor_count2)
- [按键中断电灯-EXTI](./app/interrupt/key_control_led_exti)
- [旋转编码器计次](./app/interrupt/rotary_encoder_count)
- [系统定时器中断](./app/interrupt/sys_timer_interrupt)
- [定时器中断计数-秒](./app/interrupt/timer_interrupt_count_by_seces)
- [定时器中断计数-赫兹](./app/interrupt/timer_interrupt_count_by_hz)
- [定时器外部时钟](./app/interrupt/timer_external_clock)
- [RTC 告警中断闪烁 LED](./app/interrupt/rtc_alarm_blinky_irq)

### 端口重映射

- [禁用 JTAG 端口](./app/port_remap/disable_jtag_ports)

### PWM 脉冲宽度调制

- [PWM 驱动呼吸灯](./app/pwm/pwm_led)
- [自定义引脚组合的 PWM 输出](./app/pwm/pwm_custom)
- [PWM 驱动呼吸灯-端口重映射](./app/pwm/pwm_led_remap)
- [PWM 驱动舵机](./app/pwm/pwm_driven_servo)
- [PWM 驱动直流电机](./app/pwm/pwm_driven_motor)
- [输入捕获模式测频率占空比](./app/pwm/pwm_input_capture_freq_duty_cycle)
- [旋转编码器接口计数](./app/pwm/pwm_rotary_encoder_count)
- [旋转编码器接口延时测速](./app/pwm/pwm_rotary_encoder_speed)
- [旋转编码器接口定时器测速](./app/pwm/pwm_rotary_encoder_timer_speed)

### ADC 模数转换器

- [AD 单通道](./app/adc/ad_single_channel)
- [AD 多通道](./app/adc/ad_multichannel)

### DMA 数据转运

- [打印内存地址](./app/dma/print_memory_address)
- [DMA 数据转运](./app/dma/dma_data_transfer)
- [DMA 数据连续转运](./app/dma/dma_data_continuous_transfer)
- [DMA+AD 多通道](./app/dma/scan_dma_and_ad_multichannel)
- [DMA+AD 多通道循环读取](./app/dma/scan_dma_and_ad_multichannel_loop)
- [DMA+AD 多通道分批读取](./app/dma/scan_dma_and_ad_multichannel_peek)

### USART 串行接口

- [串行接口配置](./app/usart/serial_config)
- [串行接口发送与接收](./app/usart/serial_tx_and_rx)
- [串行接口重新配置](./app/usart/serial_reconfigure)
- [串行接口写入格式化字符串](./app/usart/serial_fmt)
- [串行接口连续发送与接收](./app/usart/serial_continuous_tx_and_rx)
- [串行接口中断](./app/usart/serial_interrupt_idle)
- [串行接口收发 HEX 数据包](./app/usart/serial_hex_packet)
- [串行接口收发文本数据包](./app/usart/serial_text_packet)

### I2C 通信

- [I2C OLED 显示字符](./app/i2c/i2c_oled_show_character)
- [I2C 软件读写 MPU6050](./app/i2c/i2c_soft_mpu6050)
- [I2C 硬件读写 MPU6050](./app/i2c/i2c_hard_mpu6050)
- [I2C MPU6050 crate 读写](./app/i2c/i2c_mpu6050_crate)

### SPI 通信

- x[SPI 软件读写 W25Q64](./app/spi/spi_soft_w25q64)
- x[SPI 硬件读写 W25Q64](./app/spi/spi_hard_w25q64)
- x[w25q crate 读写 W25Q64](./app/spi/spi_w25q_crate)

### RTC

- [RTC 实时时钟计数器](./app/rtc/rtc_counter)
- [BKP 断电恢复](./app/rtc/rtc_bkp)
- [读写备份寄存器](./app/rtc/rtc_bkp_dyn_data)
- [RTC 告警闪烁 LED](./app/rtc/rtc_alarm_blinky)
- [RTC 实时时间](./app/rtc/rtc_time)

### PWR 电源控制

- [修改系统时钟主频](./app/pwr/syst_freq)
- [睡眠模式-串口发送接收](./app/pwr/sleep_mode_serial_tx_and_rx)
- [停止模式-对射式红外传感器计次](./app/pwr/stop_mode_infrared_sensor_count)
- [待机模式-实时时钟计数](./app/pwr/standby_mode_rtc_counter)

### WDG 看门狗

- [独立看门狗](./app/wdg/iwdg)
- [窗口看门狗](./app/wdg/wwdg)

### RITC 实时中断驱动并发

- [任务调度-RTIC](./app/rtic/spawn_task_rtic)
- x[EXTI 中断-RTIC](./app/rtic/exti_rtic)
- x[TIM 定时器中断-RTIC](./app/rtic/timer_interrupt_rtic)

### FreeRTOS 实时操作系统

- x[FreeRTOS-闪烁](./app/free_rtos/free_rtos_blinky)

### 常用外设工具库封装

- [硬件工具库](./core/hardware)
- [FFI Hello](./core/ffi_hello)
- [Bindgen Hello](./core/bindgen_hello)
- [Stm32f10x Rust 绑定](./core/stm32f10x_rs)

### 其他示例

- [HAL 库示例](https://github.com/stm32-rs/stm32f1xx-hal/tree/master/examples)

## 相关文档

- [Archlinux 环境搭建](./docs/Archlinux环境搭建.md)
- [编译与部署](./docs/编译与部署.md)
- [术语介绍](./docs/术语介绍.md)
- [GDB 调试](./docs/GDB调试.md)
- [Renode 仿真模拟](./docs/Renode仿真模拟.md)
- [Linux st-link 配置](./docs/Linux%20st-link配置.md)
- [Defmt 单元测试](./docs/Defmt%20单元测试.md)
- [Openocd 使用指南](./docs/Openocd使用指南.md)
- [Minicom 使用文档](./docs/Minicom使用文档.md)
- [Q&A](./docs/Q&A.md)

## 参考文档

- [官方嵌入式](https://www.rust-lang.org/zh-CN/what/embedded)
- [stm32f1xx-hal](https://github.com/stm32-rs/stm32f1xx-hal)
- [Rust 嵌入式精选列表](https://github.com/rust-embedded/awesome-embedded-rust)
- [STM32F103c8 数据表](https://www.st.com/resource/en/datasheet/cd00161566.pdf)
- [嵌入经济学](https://docs.rust-embedded.org/embedonomicon/)
- [Linux 系统下使用 cutecom 进行串口通信（一）](https://zhuanlan.zhihu.com/p/371813518)
- [Linux 下 minicom 串口助手的使用](https://www.cnblogs.com/xingboy/p/16538932.html)
- [实时中断驱动的并发](https://rtic.rs/2/book/en/)
