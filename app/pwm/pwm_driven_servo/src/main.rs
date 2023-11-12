#![no_std]
#![no_main]
#![allow(clippy::empty_loop)]

use hardware::oled;

use defmt::println;
use defmt_rtt as _;
use panic_probe as _;

use cortex_m::prelude::_embedded_hal_blocking_delay_DelayMs;
use cortex_m_rt::entry;
use stm32f1xx_hal::prelude::{
    _fugit_RateExtU32, _stm32_hal_afio_AfioExt, _stm32_hal_flash_FlashExt, _stm32_hal_gpio_GpioExt,
};
use stm32f1xx_hal::rcc::RccExt;
use stm32f1xx_hal::time::ms;
use stm32f1xx_hal::timer::{Channel, PwmExt, Tim2NoRemap};
use stm32f1xx_hal::timer::{SysDelay, SysTimerExt};
use stm32f1xx_hal::{gpio, pac};

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
    let mut gpiob = dp.GPIOB.split();

    // 冻结系统中所有时钟的配置，并将冻结的频率存储在时钟中
    let clocks = rcc
        .cfgr
        // .use_hse(8.MHz())
        // 设置系统时钟
        // .sysclk(72.MHz())
        // .pclk1(36.MHz())
        .hclk(8.MHz())
        .freeze(&mut flash.acr);

    // 具有自定义精度的阻塞延迟函数
    let mut delay = syst.delay(&clocks);

    // 初始化 OLED 显示屏
    let mut oled = oled::simple::init_oled(gpiob.pb8, gpiob.pb9, &mut gpiob.crh);

    // 按键
    println!("load key...");
    let mut key = gpiob.pb1.into_pull_up_input(&mut gpiob.crl);

    // 复用推挽输出
    println!("load servo...");
    let c1 = gpioa.pa1.into_alternate_push_pull(&mut gpioa.crl);
    let pins = c1;

    println!("load pwm...");
    let mut pwm = tim2.pwm_hz::<Tim2NoRemap, _, _>(pins, &mut afio.mapr, 1.kHz(), &clocks);

    // Enable clock on each of the channels
    // https://docs.rs/stm32f1xx-hal/0.10.0/stm32f1xx_hal/timer/index.html
    pwm.enable(Channel::C2);

    // Adjust period
    // 总周期为20ms max_duty=53333
    pwm.set_period(ms(20).into_rate());

    // Return to the original frequency
    // pwm.set_period(50.Hz()); // max_duty=53333

    let max_duty = pwm.get_max_duty();
    println!("max_duty={:?}", max_duty);

    let mut angle = 0.0;
    oled.show_string(1, 1, "Angle:");
    oled.show_string(2, 1, "Duty:");
    loop {
        let key_num = get_key_num(&mut key, &mut delay);
        if key_num == 0 {
            continue;
        }
        angle += 30.0;
        if angle > 180.0 {
            angle = 0.0
        }
        // 53333.0  20ms
        // 1333.3   0.5ms  -90度
        // 3999.9   1.5ms   0度
        // 5333.3   2ms     45度
        // 6666.6   2.5ms   90度
        // 缩放: (6666.6-5333.3)/(90-45) = 29.6
        let duty = (angle * 29.6 + 1333.3) as u16;
        oled.show_num(1, 7, angle as u32, 5);
        oled.show_num(2, 6, duty.into(), 5);
        pwm.set_duty(Channel::C2, duty);
    }
}

/// 获取按键的值
fn get_key_num(
    key1: &mut gpio::Pin<'B', 1, gpio::Input<gpio::PullUp>>,
    delay: &mut SysDelay,
) -> i32 {
    let mut key_num = 0;

    if key1.is_low() {
        // 按键按下抖动
        delay.delay_ms(20_u16);
        // 按着不动, 松手后跳出循环
        while key1.is_low() {}
        // 按键松开抖动
        delay.delay_ms(20_u16);

        key_num = 1;
    }
    key_num
}
