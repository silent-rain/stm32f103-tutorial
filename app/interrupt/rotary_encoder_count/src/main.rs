#![no_std]
#![no_main]
#![allow(clippy::empty_loop)]

use core::mem::MaybeUninit;

use hardware::oled;

use defmt::println;
use defmt_rtt as _;
use panic_probe as _;

use cortex_m::peripheral::NVIC;
use cortex_m::prelude::_embedded_hal_blocking_delay_DelayMs;
use cortex_m_rt::entry;
use stm32f1xx_hal::afio;
use stm32f1xx_hal::gpio::{self, gpiob, Edge, ExtiPin, Input, PullUp};
use stm32f1xx_hal::pac::{self, interrupt};
use stm32f1xx_hal::prelude::{
    _stm32_hal_afio_AfioExt, _stm32_hal_flash_FlashExt, _stm32_hal_gpio_GpioExt,
};
use stm32f1xx_hal::rcc::RccExt;
use stm32f1xx_hal::timer::SysTimerExt;

static mut ROTARY_ENCODER_S1: MaybeUninit<gpiob::PB0<Input<PullUp>>> = MaybeUninit::uninit();
static mut ROTARY_ENCODER_S2: MaybeUninit<gpiob::PB1<Input<PullUp>>> = MaybeUninit::uninit();

// 计数器
static mut SENSOR_COUNT: i32 = 0;
static mut NUM: i32 = 0;

#[entry]
fn main() -> ! {
    // 获取对外设的访问对象
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();
    let syst = cp.SYST;
    let mut afio = dp.AFIO.constrain();
    let mut exti = dp.EXTI;
    let mut nvic = cp.NVIC;

    let mut gpiob = dp.GPIOB.split();

    // 冻结系统中所有时钟的配置，并将冻结的频率存储在时钟中
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    // 具有自定义精度的阻塞延迟函数
    let mut delay = syst.delay(&clocks);

    // 上电延时
    delay.delay_ms(20u16);

    // 初始化旋转编码器
    init_rotary_encoder(gpiob.pb0, gpiob.pb1, &mut gpiob.crl, &mut afio, &mut exti);

    unsafe {
        // Enable interruptions
        NVIC::unmask(interrupt::EXTI0);
        NVIC::unmask(interrupt::EXTI1);
        // 设置中断的优先级
        nvic.set_priority(interrupt::EXTI0, 1);
        nvic.set_priority(interrupt::EXTI1, 2);
    }

    // 初始化 OLED 显示屏
    println!("load oled...");
    let mut oled = oled::simple::init_oled(gpiob.pb8, gpiob.pb9, &mut gpiob.crh);

    oled.show_string(1, 1, "Num:");
    loop {
        let num = unsafe {
            NUM += get_rotary_encoder_count();
            NUM
        };
        oled.show_signed_num(1, 5, num, 5);
    }
}

/// 中断调用函数
#[interrupt]
fn EXTI0() {
    let rotary_encoder_s1 = unsafe { &mut *ROTARY_ENCODER_S1.as_mut_ptr() };
    let rotary_encoder_s2 = unsafe { &mut *ROTARY_ENCODER_S2.as_mut_ptr() };
    // 如果出现数据乱跳的现象，可再次判断引脚电平，以避免抖动
    if rotary_encoder_s1.check_interrupt() {
        // 反转
        if rotary_encoder_s2.is_low() {
            unsafe {
                SENSOR_COUNT -= 1;
            }
        }

        // if we don't clear this bit, the ISR would trigger indefinitely
        rotary_encoder_s1.clear_interrupt_pending_bit();
    }
}

/// 中断调用函数
#[interrupt]
fn EXTI1() {
    let rotary_encoder_s1 = unsafe { &mut *ROTARY_ENCODER_S1.as_mut_ptr() };
    let rotary_encoder_s2 = unsafe { &mut *ROTARY_ENCODER_S2.as_mut_ptr() };
    // 如果出现数据乱跳的现象，可再次判断引脚电平，以避免抖动
    if rotary_encoder_s2.check_interrupt() {
        // 正转
        if rotary_encoder_s1.is_low() {
            unsafe {
                SENSOR_COUNT += 1;
            }
        }

        // if we don't clear this bit, the ISR would trigger indefinitely
        rotary_encoder_s2.clear_interrupt_pending_bit();
    }
}

/// 初始化旋转编码器
fn init_rotary_encoder(
    pb0: gpio::Pin<'B', 0>,
    pb1: gpio::Pin<'B', 1>,
    cr: &mut gpio::Cr<'B', false>,
    afio: &mut afio::Parts,
    exti: &mut pac::EXTI,
) {
    // 配置上拉输入, 无需配置速度
    let mut pin0 = pb0.into_pull_up_input(cr);
    let mut pin1 = pb1.into_pull_up_input(cr);

    // 配置 AFIO 外部中断引脚选择
    pin0.make_interrupt_source(afio);
    // 从该引脚启用外部中断
    pin0.enable_interrupt(exti);
    // 在下升沿生成中断
    pin0.trigger_on_edge(exti, Edge::Falling);

    // 配置 AFIO 外部中断引脚选择
    pin1.make_interrupt_source(afio);
    // 从该引脚启用外部中断
    pin1.enable_interrupt(exti);
    // 在下升沿生成中断
    pin1.trigger_on_edge(exti, Edge::Falling);

    let rotary_encoder_s1 = unsafe { &mut *ROTARY_ENCODER_S1.as_mut_ptr() };
    *rotary_encoder_s1 = pin0;

    let rotary_encoder_s2 = unsafe { &mut *ROTARY_ENCODER_S2.as_mut_ptr() };
    *rotary_encoder_s2 = pin1;
}

/// 获取传感器计数
fn get_rotary_encoder_count() -> i32 {
    unsafe {
        let tmp = SENSOR_COUNT;
        SENSOR_COUNT = 0;
        tmp
    }
}
