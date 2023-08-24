# STM32F103 开发板使用案例

这是一个关于 STM32F103 开发板使用案例，记录一些使用 Rust 语言进行开发测试的案例。

该教程将主要根据 [B 站江科大](https://www.bilibili.com/video/BV1th411z7sn?p=1&vd_source=c459b4f4f90bc42bb5ddb5baf12e1bc7) 的视频教学进行学习嵌入式开发。示例中将会借用 `B 站江科大` 视频配套资料中的一些资源进行展示。

## 示例目录

- [Hello World](./app/helloworld)
- [自定义异常 Panic](./app/panics)
- [测试套件](./app/testsuite)
- [点灯](./app/turns_user_led)
- [点灯-RTT](./app/turns_user_led_rtt)
- [计时器闪烁 LED](./app/timer_blinky)
- [延迟闪烁 LED](./app/delay_blinky)
- [TIM2 通用定时器延迟闪烁 LED](./app/tim2_timer_delay_blinky)
- [闪烁 LED](./app/blinky)
- [LED 流水灯](./app/led_flow_light)
- [蜂鸣器](./app/buzzer)
- [按键控制 LED](./app/key_control_led)
- [光敏传感器控制蜂鸣器](./app/light_sensor_control_buzzer)
- [OLED I2C 通信协议显示字符](./app/oled_i2c_show_character)
- [对射式红外传感器计次](./app/opposing_infrared_sensor_count)
- [对射式红外传感器计次 2](./app/opposing_infrared_sensor_count2)
- [按键中断电灯-EXTI](./app/key_control_led_exti)
- [旋转编码器计次](./app/rotary_encoder_count)
- [系统定时器中断](./app/sys_timer_interrupt)
- [定时器中断](./app/timer_interrupt)
- [定时器中断 2](./app/timer_interrupt2)
- x[定时器外部时钟](./app/timer_external_clock)
- [PWM 驱动呼吸灯](./app/pwm_led)
- [自定义引脚组合的 PWM 输出](./app/pwm_custom)
- [PWM 驱动呼吸灯-端口重映射](./app/pwm_led_remap)

## 相关文档

- [Archlinux 环境搭建](./docs/Archlinux环境搭建.md)
- [编译与部署](./docs/编译与部署.md)
- [术语介绍](./docs/术语介绍.md)
- [GDB 调试](./docs/GDB调试.md)
- [Renode 仿真模拟](./docs/Renode仿真模拟.md)

## 参考文档

- [官方嵌入式](https://www.rust-lang.org/zh-CN/what/embedded)
- [stm32f1xx-hal](https://github.com/stm32-rs/stm32f1xx-hal)
- [STM32F103c8 数据表](https://www.st.com/resource/en/datasheet/cd00161566.pdf)
- [嵌入经济学](https://docs.rust-embedded.org/embedonomicon/)
