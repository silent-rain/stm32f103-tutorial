//! OLED I2C 通信协议显示字符

#![allow(clippy::empty_loop)]
#![deny(unsafe_code)]
#![no_std]
#![no_main]

use hardware::oled;

use defmt_rtt as _;
use panic_probe as _;

use cortex_m::prelude::_embedded_hal_blocking_delay_DelayMs;
use cortex_m_rt::entry;
use stm32f1xx_hal::flash::{self, FlashExt};
use stm32f1xx_hal::gpio::{self, gpioa, gpiob, GpioExt, OutputSpeed};
use stm32f1xx_hal::pac;
use stm32f1xx_hal::rcc::{self, RccExt};
use stm32f1xx_hal::timer::{SysDelay, SysTimerExt};

#[entry]
fn main() -> ! {
    // 初始化外设
    let (flash, rcc, system_timer, _gpioa, gpiob) = init_peripheral();

    // 封装具有自定义精度的阻塞延迟函数
    let mut delay = sys_delay(flash, rcc, system_timer);

    // 上电延时
    delay.delay_ms(20u16);

    // 初始化 OLED 显示屏
    let (mut scl, mut sda) = init_oled(gpiob);

    // 初始化 OLED 配置
    oled::init_oled_config(&mut scl, &mut sda);

    oled::show_char(&mut scl, &mut sda, 1, 1, 'A');
    oled::show_string(&mut scl, &mut sda, 1, 3, "HelloWorld!");
    oled::show_num(&mut scl, &mut sda, 2, 1, 12345, 5);
    oled::show_signed_num(&mut scl, &mut sda, 2, 7, -66, 2);
    oled::show_hex_num(&mut scl, &mut sda, 3, 1, 0xAA55, 4);
    oled::show_bin_num(&mut scl, &mut sda, 4, 1, 0xAA55, 16);

    loop {}
}

/// 初始化外设
fn init_peripheral() -> (
    flash::Parts,
    rcc::Rcc,
    cortex_m::peripheral::SYST,
    gpioa::Parts,
    gpiob::Parts,
) {
    // 获取对外设的访问对象
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();
    let flash: flash::Parts = dp.FLASH.constrain();
    let rcc: rcc::Rcc = dp.RCC.constrain();
    let system_timer = cp.SYST;
    let gpioa: gpioa::Parts = dp.GPIOA.split();
    let gpiob: gpiob::Parts = dp.GPIOB.split();
    (flash, rcc, system_timer, gpioa, gpiob)
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

/// 初始化 OLED 显示屏
/// 端口初始化
/// 将引脚配置为作为开漏输出模式
/// 输入模式中速度是没有用的, 无需配置
fn init_oled(
    mut gpiob: gpiob::Parts,
) -> (
    gpio::PB8<gpio::Output<gpio::OpenDrain>>,
    gpio::PB9<gpio::Output<gpio::OpenDrain>>,
) {
    let mut scl = gpiob.pb8.into_open_drain_output(&mut gpiob.crh);
    let mut sda = gpiob.pb9.into_open_drain_output(&mut gpiob.crh);
    scl.set_speed(&mut gpiob.crh, gpio::IOPinSpeed::Mhz50);
    sda.set_speed(&mut gpiob.crh, gpio::IOPinSpeed::Mhz50);
    scl.set_high();
    sda.set_high();
    (scl, sda)
}
