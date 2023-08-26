#![allow(clippy::empty_loop)]
#![no_std]
#![no_main]

mod hardware;
use hardware::oled;
use hardware::peripheral::Peripheral;

use defmt::println;
use defmt_rtt as _;
use panic_probe as _;

use cortex_m::prelude::_embedded_hal_blocking_delay_DelayMs;
use cortex_m_rt::entry;
use stm32f1xx_hal::gpio::{self, Alternate, IOPinSpeed, OutputSpeed};
use stm32f1xx_hal::pac::TIM2;
use stm32f1xx_hal::prelude::_fugit_RateExtU32;
use stm32f1xx_hal::timer::{Ch, PwmHz, SysDelay};
use stm32f1xx_hal::timer::{Channel, PwmExt, Tim2NoRemap};

#[entry]
fn main() -> ! {
    // 初始化外设
    let Peripheral {
        mut flash,
        rcc,
        tim2,
        syst,
        mut afio,
        exti: _,
        nvic: _,
        mut gpioa,
        mut gpiob,
    } = Peripheral::new();

    // 冻结系统中所有时钟的配置，并将冻结的频率存储在时钟中
    let clocks = rcc
        .cfgr
        // .use_hse(8.MHz())
        // 设置系统时钟
        // .sysclk(72.MHz())
        // .pclk1(36.MHz())
        .hclk(8.MHz())
        .freeze(&mut flash.acr);

    // 封装具有自定义精度的阻塞延迟函数
    let mut delay = Peripheral::sys_delay(&mut flash, &clocks, syst);

    // 初始化 OLED 显示屏
    println!("load oled...");
    let (mut scl, mut sda) = oled::init_oled(gpiob.pb8, gpiob.pb9, &mut gpiob.crh);

    // 按键
    println!("load key...");
    let mut key = gpiob.pb1.into_pull_up_input(&mut gpiob.crl);

    // 直流电机方向引脚
    println!("load motor...");
    let mut ain1: gpio::Pin<'A', 4, gpio::Output> = gpioa.pa4.into_push_pull_output(&mut gpioa.crl);
    let mut ain2 = gpioa.pa5.into_push_pull_output(&mut gpioa.crl);
    ain1.set_speed(&mut gpioa.crl, IOPinSpeed::Mhz50);
    ain2.set_speed(&mut gpioa.crl, IOPinSpeed::Mhz50);

    // pwma 速度控制引脚
    println!("load pwma...");
    let c3 = gpioa.pa2.into_alternate_push_pull(&mut gpioa.crl);
    let pins = c3;

    println!("load pwm...");
    let mut pwm = tim2.pwm_hz::<Tim2NoRemap, _, _>(pins, &mut afio.mapr, 1.kHz(), &clocks);

    // Enable clock on each of the channels
    // https://docs.rs/stm32f1xx-hal/0.10.0/stm32f1xx_hal/timer/index.html
    pwm.enable(Channel::C3);

    // Return to the original frequency
    pwm.set_period(1.kHz());

    let max_duty = pwm.get_max_duty();
    println!("max_duty={:?}", max_duty);

    let mut speed = 0;
    oled::show_string(&mut scl, &mut sda, 1, 1, "Speed:");
    loop {
        let key_num = get_key_num(&mut key, &mut delay);
        if key_num == 1 {
            speed += 20;
            if speed > 100 {
                speed = -100;
            }
        }
        set_speed(&mut ain1, &mut ain2, &mut pwm, 20);
        oled::show_signed_num(&mut scl, &mut sda, 1, 7, speed, 3);
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

/// 设置直流电机速度
fn set_speed(
    ain1: &mut gpio::Pin<'A', 4, gpio::Output>,
    ain2: &mut gpio::Pin<'A', 5, gpio::Output>,
    pwm: &mut PwmHz<TIM2, Tim2NoRemap, Ch<2>, gpio::Pin<'A', 2, Alternate>>,
    speed: i32,
) {
    // 正转
    if speed >= 0 {
        ain1.set_high();
        ain2.set_low();
    } else {
        ain2.set_high();
        ain1.set_low();
    }
    pwm.set_duty(Channel::C3, speed as u16);
}
