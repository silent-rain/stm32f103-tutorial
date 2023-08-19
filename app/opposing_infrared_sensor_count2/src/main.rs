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
use stm32f1xx_hal::gpio::{self, gpiob, Edge, ExtiPin, Input, PullUp};
use stm32f1xx_hal::pac::{self, interrupt};

/// 对射式红外传感器
/// 这个属于ISR所有。
/// main（）只能在初始化阶段访问它们，在初始化阶段中断尚未启用（即不能发生并发访问）。
/// 启用中断后，main（）可能不再对这些对象有任何引用。
/// 出于极简主义的考虑，我们在这里不使用RTIC，这将是更好的方式。
static mut INFRARED_SENSOR: MaybeUninit<gpiob::PB14<Input<PullUp>>> = MaybeUninit::uninit();

// 计数器
static mut COUNT_SENSOR_COUNT: u32 = 0;

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
        gpioa: _,
        mut gpiob,
    } = Peripheral::new();

    // 封装具有自定义精度的阻塞延迟函数
    let mut delay = Peripheral::sys_delay(flash, rcc, syst);

    // 上电延时
    delay.delay_ms(20u16);

    // 初始化对射式红外传感器
    init_infrared_sensor(gpiob.pb14, &mut gpiob.crh, &mut afio, &mut exti);

    unsafe {
        // Enable EXTI15_10 interruptions
        NVIC::unmask(interrupt::EXTI15_10);
        // 将中断的“优先级”设置为prio
        nvic.set_priority(interrupt::EXTI15_10, 0x80);
    }

    // 初始化 OLED 显示屏
    rprintln!("load oled...");
    let (mut scl, mut sda) = oled::init_oled(gpiob.pb8, gpiob.pb9, &mut gpiob.crh);

    oled::show_string(&mut scl, &mut sda, 1, 1, "Count:");
    loop {
        oled::show_num(&mut scl, &mut sda, 1, 7, get_sensor_count(), 5);
    }
}

/// 中断调用函数
#[interrupt]
fn EXTI15_10() {
    let infrared_sensor = unsafe { &mut *INFRARED_SENSOR.as_mut_ptr() };

    if infrared_sensor.check_interrupt() {
        unsafe {
            COUNT_SENSOR_COUNT += 1;
        }

        // if we don't clear this bit, the ISR would trigger indefinitely
        infrared_sensor.clear_interrupt_pending_bit();
    }
}

/// 初始化对射式红外传感器
fn init_infrared_sensor(
    pb14: gpio::Pin<'B', 14>,
    crh: &mut gpio::Cr<'B', true>,
    afio: &mut stm32f1xx_hal::afio::Parts,
    exti: &mut pac::EXTI,
) {
    // 配置上拉输入, 无需配置速度
    let mut pin = pb14.into_pull_up_input(crh);
    // 配置 AFIO 外部中断引脚选择
    pin.make_interrupt_source(afio);
    // 从该引脚启用外部中断
    pin.enable_interrupt(exti);
    // 在上升沿生成中断
    pin.trigger_on_edge(exti, Edge::Rising);

    let infrared_sensor = unsafe { &mut *INFRARED_SENSOR.as_mut_ptr() };
    *infrared_sensor = pin;
}

/// 获取传感器计数
fn get_sensor_count() -> u32 {
    unsafe { COUNT_SENSOR_COUNT }
}
