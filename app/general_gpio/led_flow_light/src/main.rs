//! Blinks several LEDs stored in an array

#![no_std]
#![no_main]
#![deny(unsafe_code)]

use defmt_rtt as _;
use panic_probe as _;

use cortex_m_rt::entry;
use stm32f1xx_hal::pac;
use stm32f1xx_hal::prelude::_fugit_ExtU32;
use stm32f1xx_hal::prelude::_stm32_hal_flash_FlashExt;
use stm32f1xx_hal::prelude::_stm32_hal_gpio_GpioExt;
use stm32f1xx_hal::rcc::RccExt;
use stm32f1xx_hal::timer::SysTimerExt;

#[entry]
fn main() -> ! {
    // 获取对外设的访问对象
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    // 获得原始flash和rcc设备的所有权，并将它们转换为相应的HAL结构
    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();

    // 冻结系统中所有时钟的配置，并将冻结的频率存储在时钟中
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    // 具有自定义精度的阻塞延迟
    let mut delay = cp.SYST.delay(&clocks);

    // 获取GPIO外围设备
    let mut gpioa = dp.GPIOA.split();

    // 创建要闪烁的LED阵列
    let mut leds = [
        gpioa.pa0.into_push_pull_output(&mut gpioa.crl).erase(),
        gpioa.pa1.into_push_pull_output(&mut gpioa.crl).erase(),
        gpioa.pa2.into_push_pull_output(&mut gpioa.crl).erase(),
        gpioa.pa3.into_push_pull_output(&mut gpioa.crl).erase(),
        gpioa.pa4.into_push_pull_output(&mut gpioa.crl).erase(),
        gpioa.pa5.into_push_pull_output(&mut gpioa.crl).erase(),
        gpioa.pa6.into_push_pull_output(&mut gpioa.crl).erase(),
        gpioa.pa7.into_push_pull_output(&mut gpioa.crl).erase(),
    ];

    loop {
        for led in leds.iter_mut() {
            led.set_high();
            delay.delay(500.millis());
            led.set_low();
            delay.delay(500.millis());
        }
    }
}
