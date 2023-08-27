#![allow(clippy::empty_loop)]
#![no_std]
#![no_main]

mod hardware;
use hardware::oled;
use hardware::peripheral::Peripheral;

use core::mem::MaybeUninit;

// 用于处理错误情况
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

use cortex_m::peripheral::NVIC;
use cortex_m::prelude::_embedded_hal_blocking_delay_DelayMs;
use cortex_m_rt::entry;
use stm32f1xx_hal::afio;
use stm32f1xx_hal::gpio::{
    self, gpioa, gpiob, Edge, ExtiPin, Input, Output, OutputSpeed, PullUp, PushPull,
};
use stm32f1xx_hal::pac::{self, interrupt};

static mut LED1: MaybeUninit<gpioa::PA1<Output<PushPull>>> = MaybeUninit::uninit();
static mut LED2: MaybeUninit<gpioa::PA2<Output<PushPull>>> = MaybeUninit::uninit();
static mut KEY1: MaybeUninit<gpiob::PB1<Input<PullUp>>> = MaybeUninit::uninit();
static mut KEY11: MaybeUninit<gpiob::PB11<Input<PullUp>>> = MaybeUninit::uninit();

// 计数器
static mut COUNT: u32 = 0;

#[entry]
fn main() -> ! {
    rtt_init_print!();

    // 初始化外设
    let Peripheral {
        flash,
        rcc,
        syst,
        mut afio,
        mut exti,
        mut nvic,
        mut gpioa,
        mut gpiob,
    } = Peripheral::new();

    // 封装具有自定义精度的阻塞延迟函数
    let mut delay = Peripheral::sys_delay(flash, rcc, syst);

    // 上电延时
    delay.delay_ms(20u16);

    // 初始化 LED
    init_led(gpioa.pa1, gpioa.pa2, &mut gpioa.crl);

    // 初始化按键
    init_key(
        gpiob.pb1,
        gpiob.pb11,
        &mut gpiob.crl,
        &mut gpiob.crh,
        &mut afio,
        &mut exti,
    );

    unsafe {
        // Enable interruptions
        NVIC::unmask(interrupt::EXTI1);
        NVIC::unmask(interrupt::EXTI15_10);
        // 设置中断的优先级
        nvic.set_priority(interrupt::EXTI1, 1);
        nvic.set_priority(interrupt::EXTI15_10, 2);
    }

    // 初始化 OLED 显示屏
    rprintln!("load oled...");
    let (mut scl, mut sda) = oled::init_oled(gpiob.pb8, gpiob.pb9, &mut gpiob.crh);

    oled::show_string(&mut scl, &mut sda, 1, 1, "Count:");
    loop {
        oled::show_num(&mut scl, &mut sda, 1, 7, get_key_count(), 5);
    }
}

/// key1 中断调用函数
#[interrupt]
fn EXTI1() {
    let led1 = unsafe { &mut *LED1.as_mut_ptr() };
    let key1 = unsafe { &mut *KEY1.as_mut_ptr() };

    if key1.check_interrupt() {
        led1.toggle();
        unsafe { COUNT += 1 }

        // if we don't clear this bit, the ISR would trigger indefinitely
        key1.clear_interrupt_pending_bit();
    }
}

/// key2 中断调用函数
#[interrupt]
fn EXTI15_10() {
    let led2 = unsafe { &mut *LED2.as_mut_ptr() };
    let key11 = unsafe { &mut *KEY11.as_mut_ptr() };

    if key11.check_interrupt() {
        led2.toggle();
        unsafe { COUNT += 1 }

        // if we don't clear this bit, the ISR would trigger indefinitely
        key11.clear_interrupt_pending_bit();
    }
}

/// 初始化 LED
fn init_led(pa1: gpio::Pin<'A', 1>, pa2: gpio::Pin<'A', 2>, cr: &mut gpio::Cr<'A', false>) {
    // 推挽输出
    let mut pin1 = pa1.into_push_pull_output_with_state(cr, gpio::PinState::High);
    let mut pin2 = pa2.into_push_pull_output_with_state(cr, gpio::PinState::High);
    // 设置其输出速度（50 MHz）。
    pin1.set_speed(cr, gpio::IOPinSpeed::Mhz50);
    pin2.set_speed(cr, gpio::IOPinSpeed::Mhz50);

    let led1 = unsafe { &mut *LED1.as_mut_ptr() };
    *led1 = pin1;

    let led2 = unsafe { &mut *LED2.as_mut_ptr() };
    *led2 = pin2;
}

/// 初始化按键
fn init_key(
    pb1: gpio::Pin<'B', 1>,
    pb11: gpio::Pin<'B', 11>,
    crl: &mut gpio::Cr<'B', false>,
    crh: &mut gpio::Cr<'B', true>,
    afio: &mut afio::Parts,
    exti: &mut pac::EXTI,
) {
    let mut key1 = pb1.into_pull_up_input(crl);
    let mut key11 = pb11.into_pull_up_input(crh);

    // 配置 AFIO 外部中断引脚选择
    key1.make_interrupt_source(afio);
    // 从该引脚启用外部中断
    key1.enable_interrupt(exti);
    // 下升沿生成中断
    key1.trigger_on_edge(exti, Edge::RisingFalling);

    // 配置 AFIO 外部中断引脚选择
    key11.make_interrupt_source(afio);
    // 从该引脚启用外部中断
    key11.enable_interrupt(exti);
    // 在上升沿生成中断
    key11.trigger_on_edge(exti, Edge::RisingFalling);

    let ukey1 = unsafe { &mut *KEY1.as_mut_ptr() };
    *ukey1 = key1;

    let ukey11 = unsafe { &mut *KEY11.as_mut_ptr() };
    *ukey11 = key11;
}

/// 获取按键计数
fn get_key_count() -> u32 {
    unsafe { COUNT }
}
