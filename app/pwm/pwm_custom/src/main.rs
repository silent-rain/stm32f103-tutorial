#![allow(clippy::empty_loop)]
#![no_std]
#![no_main]

mod hardware;
use cortex_m::asm;
use hardware::peripheral::Peripheral;

use defmt_rtt as _;
use panic_probe as _;

use cortex_m_rt::entry;
use defmt::println;
use stm32f1xx_hal::prelude::_fugit_RateExtU32;
use stm32f1xx_hal::timer::Timer;

#[entry]
fn main() -> ! {
    // 初始化外设
    let Peripheral {
        mut flash,
        rcc,
        tim2: _,
        tim3,
        syst: _,
        mut afio,
        exti: _,
        nvic: _,
        gpioa,
        mut gpiob,
    } = Peripheral::new();

    // 冻结系统中所有时钟的配置，并将冻结的频率存储在时钟中
    let clocks = rcc
        .cfgr
        // .use_hse(8.MHz())
        // 设置系统时钟
        // .sysclk(72.MHz())
        // .pclk1(36.MHz())
        .freeze(&mut flash.acr);

    let (_pa15, _pb3, pb4) = afio.mapr.disable_jtag(gpioa.pa15, gpiob.pb3, gpiob.pb4);

    // TIM3
    let p0 = pb4.into_alternate_push_pull(&mut gpiob.crl);
    let p1 = gpiob.pb5.into_alternate_push_pull(&mut gpiob.crl);
    let pins = (p0, p1);

    println!("load pwm...");

    let pwm = Timer::new(tim3, &clocks).pwm_hz(pins, &mut afio.mapr, 1.kHz());

    let max = pwm.get_max_duty();

    let mut pwm_channels = pwm.split();

    // Enable the individual channels
    pwm_channels.0.enable();
    pwm_channels.1.enable();

    // full
    pwm_channels.0.set_duty(max);
    pwm_channels.1.set_duty(max);

    asm::bkpt();

    // dim
    pwm_channels.1.set_duty(max / 4);

    asm::bkpt();

    // zero
    pwm_channels.0.set_duty(0);
    pwm_channels.1.set_duty(0);

    asm::bkpt();

    loop {}
}
