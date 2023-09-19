//! 点灯

#![no_std]
#![no_main]
// 允许有空的循环结构；
#![allow(clippy::empty_loop)]
// 禁止使用 Rust 的 unsafe 代码；
#![deny(unsafe_code)]

// 用于处理错误情况；
use defmt_rtt as _;
use panic_probe as _;

// 日志打印
use defmt::println;
// 用于标记程序入口；
use cortex_m_rt::entry;
// 可用于输出和相关AlternateMode引脚的回转速率
use stm32f1xx_hal::gpio::IOPinSpeed;
// 用于设置IO引脚的转换速率; 最初，所有引脚都设置为最大转换速率
use stm32f1xx_hal::gpio::OutputSpeed;
// 微控制器的外围访问API
use stm32f1xx_hal::pac;
// 在独立引脚和寄存器中拆分GPIO外设的扩展特性
use stm32f1xx_hal::prelude::_stm32_hal_gpio_GpioExt;

// 标记接下来的函数是程序的入口点；
#[entry]
fn main() -> ! {
    // 获取对特定设备外设的访问权限
    let p = pac::Peripherals::take().unwrap();

    // 通过拆分 GPIOA 区块，获取对其各引脚的互斥访问。
    let mut gpioa = p.GPIOA.split();

    // 将 PA0 引脚配置为推挽式输出。
    let mut led = gpioa.pa0.into_push_pull_output(&mut gpioa.crl);
    // 设置其输出速度（50 MHz）。
    // 然后在接下来的代码中，我们将使用该引脚来控制 LED 的状态。
    led.set_speed(&mut gpioa.crl, IOPinSpeed::Mhz50);

    println!("open led...");
    // 设置低电平
    // led.set_low();
    led.set_high();

    // 无限循环
    loop {}
}
