//! Demonstrate the use of a blocking `Delay` using TIM2 general-purpose timer.

#![no_std]
#![no_main]
#![deny(unsafe_code)]

use defmt_rtt as _;
use panic_probe as _;

use cortex_m::prelude::_embedded_hal_blocking_delay_DelayMs;
use cortex_m_rt::entry;
use stm32f1xx_hal::gpio::IOPinSpeed;
use stm32f1xx_hal::gpio::OutputSpeed;
use stm32f1xx_hal::pac;
use stm32f1xx_hal::prelude::_fugit_ExtU32;
use stm32f1xx_hal::prelude::_fugit_RateExtU32;
use stm32f1xx_hal::prelude::_stm32_hal_flash_FlashExt;
use stm32f1xx_hal::prelude::_stm32_hal_gpio_GpioExt;
use stm32f1xx_hal::rcc::RccExt;
use stm32f1xx_hal::timer::TimerExt;

#[entry]
fn main() -> ! {
    // 获取对外设的访问对象
    let _cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    // 获得原始flash和rcc设备的所有权，并将它们转换为相应的HAL结构
    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();

    // 设置系统时钟。我们想在48MHz的频率下运行。
    let clocks = rcc
        .cfgr
        .use_hse(8.MHz())
        .sysclk(48.MHz())
        .freeze(&mut flash.acr);

    // 获取GPIO外围设备
    let mut gpioa = dp.GPIOA.split();

    // 将 PA0 引脚配置为推挽式输出。
    let mut led = gpioa.pa0.into_push_pull_output(&mut gpioa.crl);
    // 设置其输出速度（50 MHz）。
    // 然后在接下来的代码中，我们将使用该引脚来控制 LED 的状态。
    led.set_speed(&mut gpioa.crl, IOPinSpeed::Mhz50);

    // 基于通用pupose 32位定时器TIM2创建延迟抽象

    //let mut delay = hal::timer::FTimerUs::new(dp.TIM2, &clocks).delay();
    // or
    let mut delay = dp.TIM2.delay_us(&clocks);

    // 等待计时器触发更新并更改LED的状态
    loop {
        // On for 1s, off for 3s.
        led.set_high();
        // Use `embedded_hal::DelayMs` trait
        delay.delay_ms(1000_u32);
        led.set_low();
        // or use `fugit` duration units
        delay.delay(3.secs());
    }
}
