#![no_std]
#![no_main]
#![allow(clippy::empty_loop)]

use hardware::oled;

use defmt::println;
use defmt_rtt as _;
use panic_probe as _;

use cortex_m::prelude::_embedded_hal_blocking_delay_DelayMs;
use cortex_m_rt::entry;
use stm32f1xx_hal::pac;
use stm32f1xx_hal::prelude::_fugit_RateExtU32;
use stm32f1xx_hal::prelude::_stm32_hal_afio_AfioExt;
use stm32f1xx_hal::prelude::_stm32_hal_flash_FlashExt;
use stm32f1xx_hal::prelude::_stm32_hal_gpio_GpioExt;
use stm32f1xx_hal::rcc::RccExt;
use stm32f1xx_hal::timer::Configuration;
use stm32f1xx_hal::timer::ReadMode;
use stm32f1xx_hal::timer::SysTimerExt;
use stm32f1xx_hal::timer::Tim3NoRemap;
use stm32f1xx_hal::timer::Timer;
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
    let tim3 = dp.TIM3;
    let mut dbg = dp.DBGMCU;

    let mut gpioa = dp.GPIOA.split();
    let mut gpiob = dp.GPIOB.split();

    // 冻结系统中所有时钟的配置，并将冻结的频率存储在时钟中
    let clocks = rcc
        .cfgr
        // .use_hse(8.MHz())
        // 设置系统时钟
        // .sysclk(72.MHz())
        // .pclk1(36.MHz())
        // .hclk(72.MHz())
        .freeze(&mut flash.acr);

    // 具有自定义精度的阻塞延迟函数
    let mut delay = syst.delay(&clocks);

    // 初始化 OLED 显示屏
    println!("load oled...");
    let mut oled = oled::simple::init_oled(gpiob.pb8, gpiob.pb9, &mut gpiob.crh);

    // 待测信号输出至 PA0，PA0 通过导线输出至 PA6
    println!("load pwm...");
    let pa0 = gpioa.pa0.into_alternate_push_pull(&mut gpioa.crl);
    let mut pwm = tim2.pwm_hz::<Tim2NoRemap, _, _>(pa0, &mut afio.mapr, 1.kHz(), &clocks);
    pwm.enable(Channel::C1);
    pwm.set_period(8.kHz());

    let pa6 = gpioa.pa6;
    let pa7 = gpioa.pa7;
    let pwm_input = Timer::new(tim3, &clocks).pwm_input::<Tim3NoRemap, _>(
        (pa6, pa7),
        &mut afio.mapr,
        &mut dbg,
        Configuration::RawValues {
            arr: (65536 - 1) as u16,
            presc: 72 - 1,
        },
    );

    let freq = pwm.get_period().to_Hz();
    println!("Freq pa0={:?}", freq);

    oled.show_string(1, 1, "Freq:00000Hz");
    oled.show_string(2, 1, "Duty:00%");
    loop {
        for i in 1..20 {
            pwm.set_period(i.kHz());
            let duty = (i * 1000) as u16;
            pwm.set_duty(Channel::C1, duty);
            let freq = pwm.get_period().to_Hz();
            println!("Freq pa0 period={:?} duty={:?} freq={:?}", i, duty, freq);

            if let Ok(freq) = pwm_input.read_frequency(ReadMode::Instant, &clocks) {
                oled.show_num(1, 6, freq.to_Hz(), 5);
            }
            if let Ok(duty_cycle) = pwm_input.read_duty(ReadMode::Instant) {
                oled.show_num(2, 6, duty_cycle.0.into(), 2);
            }
            delay.delay_ms(1000_u16)
        }
    }
}
