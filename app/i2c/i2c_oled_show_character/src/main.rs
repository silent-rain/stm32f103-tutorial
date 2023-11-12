//! OLED I2C 通信协议显示字符

#![no_std]
#![no_main]
#![allow(clippy::empty_loop)]
#![deny(unsafe_code)]

use hardware::oled;

use defmt::println;
use defmt_rtt as _;
use panic_probe as _;

use cortex_m::prelude::_embedded_hal_blocking_delay_DelayMs;
use cortex_m_rt::entry;
use stm32f1xx_hal::flash::{self, FlashExt};
use stm32f1xx_hal::gpio::{gpiob, GpioExt};
use stm32f1xx_hal::pac;
use stm32f1xx_hal::rcc::{self, RccExt};
use stm32f1xx_hal::timer::{SysDelay, SysTimerExt};

#[entry]
fn main() -> ! {
    // 获取对外设的访问对象
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();
    let flash: flash::Parts = dp.FLASH.constrain();
    let rcc: rcc::Rcc = dp.RCC.constrain();
    let system_timer = cp.SYST;
    let mut gpiob: gpiob::Parts = dp.GPIOB.split();

    // 封装具有自定义精度的阻塞延迟函数
    let mut delay = sys_delay(flash, rcc, system_timer);

    // 上电延时
    delay.delay_ms(20u16);

    // 初始化 OLED 显示屏
    println!("load oled...");
    let mut oled = oled::simple::init_oled(gpiob.pb8, gpiob.pb9, &mut gpiob.crh);

    oled.show_char(1, 1, 'A');
    oled.show_string(1, 3, "HelloWorld!");
    oled.show_num(2, 1, 12345, 5);
    oled.show_signed_num(2, 7, -66, 2);
    oled.show_hex_num(3, 1, 0xAA55, 4);
    oled.show_bin_num(4, 1, 0xAA55, 16);

    loop {}
}

/// 封装具有自定义精度的阻塞延迟函数
fn sys_delay(
    mut flash: flash::Parts,
    rcc: rcc::Rcc,
    system_timer: cortex_m::peripheral::SYST,
) -> SysDelay {
    // 冻结系统中所有时钟的配置，并将冻结的频率存储在时钟中
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    // 具有自定义精度的阻塞延迟
    system_timer.delay(&clocks)
}
