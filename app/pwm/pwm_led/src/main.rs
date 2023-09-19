#![no_std]
#![no_main]
#![allow(clippy::empty_loop)]

use defmt::println;
use defmt_rtt as _;
use panic_probe as _;

use cortex_m_rt::entry;
use stm32f1xx_hal::pac;
use stm32f1xx_hal::prelude::_fugit_ExtU32;
use stm32f1xx_hal::prelude::_fugit_RateExtU32;
use stm32f1xx_hal::prelude::_stm32_hal_afio_AfioExt;
use stm32f1xx_hal::prelude::_stm32_hal_flash_FlashExt;
use stm32f1xx_hal::prelude::_stm32_hal_gpio_GpioExt;
use stm32f1xx_hal::rcc::RccExt;
use stm32f1xx_hal::timer::SysTimerExt;
use stm32f1xx_hal::timer::{Channel, PwmExt, Tim2NoRemap};

#[entry]
fn main() -> ! {
    // 获取对外设的访问对象
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();
    let syst = cp.SYST;
    let mut afio = dp.AFIO.constrain();
    let tim2 = dp.TIM2;

    let mut gpioa = dp.GPIOA.split();

    // 冻结系统中所有时钟的配置，并将冻结的频率存储在时钟中
    let clocks = rcc
        .cfgr
        // .use_hse(8.MHz())
        // 设置系统时钟
        // .sysclk(72.MHz())
        // .pclk1(36.MHz())
        .freeze(&mut flash.acr);

    // 具有自定义精度的阻塞延迟函数
    let mut delay = syst.delay(&clocks);

    // TIM2
    // 复用推挽输出
    let c1 = gpioa.pa0.into_alternate_push_pull(&mut gpioa.crl);
    let pins = c1;

    println!("load pwm...");
    //let mut pwm =
    //    Timer::new(tim2, &clocks).pwm_hz::<Tim2NoRemap, _, _>(pins, &mut afio.mapr, 1.kHz());
    // or
    let mut pwm = tim2.pwm_hz::<Tim2NoRemap, _, _>(pins, &mut afio.mapr, 1.kHz(), &clocks);

    // Enable clock on each of the channels
    pwm.enable(Channel::C1);

    // Adjust period to 0.5 seconds
    // pwm.set_period(ms(500).into_rate());

    // Return to the original frequency
    pwm.set_period(100.kHz());

    loop {
        for i in 0..=100 {
            pwm.set_duty(Channel::C1, i);
            delay.delay(10.millis());
        }
        for i in 0..=100 {
            pwm.set_duty(Channel::C1, 100 - i);
            delay.delay(10.millis());
        }
    }
}
