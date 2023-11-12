//! OLED I2C 通信协议显示字符
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
use stm32f1xx_hal::flash::{self, FlashExt};
use stm32f1xx_hal::gpio::{gpioa, gpiob, Edge, ExtiPin, GpioExt, Input, PullUp};
use stm32f1xx_hal::pac::interrupt;
use stm32f1xx_hal::prelude::_stm32_hal_afio_AfioExt;
use stm32f1xx_hal::rcc::{self, RccExt};
use stm32f1xx_hal::timer::{SysDelay, SysTimerExt};
use stm32f1xx_hal::{afio, pac};

/// 对射式红外传感器
/// 这个属于ISR所有。
/// main（）只能在初始化阶段访问它们，在初始化阶段中断尚未启用（即不能发生并发访问）。
/// 启用中断后，main（）可能不再对这些对象有任何引用。
/// 出于极简主义的考虑，我们在这里不使用RTIC，这将是更好的方式。
static mut INFRARED_SENSOR: MaybeUninit<gpiob::PB14<Input<PullUp>>> = MaybeUninit::uninit();

// 计数器
static mut SENSOR_COUNT: u32 = 0;

#[entry]
fn main() -> ! {
    // 初始化外设
    let (flash, rcc, system_timer, mut afio, mut exti, _gpioa, mut gpiob) = init_peripheral();

    // 封装具有自定义精度的阻塞延迟函数
    let mut delay = sys_delay(flash, rcc, system_timer);

    // 初始化 OLED 显示屏
    println!("load oled...");
    let mut oled = oled::simple::init_oled(gpiob.pb8, gpiob.pb9, &mut gpiob.crh);

    // 上电延时
    delay.delay_ms(20u16);

    {
        // 对射式红外传感器
        // 配置上拉输入, 无需配置速度
        let infrared_sensor = unsafe { &mut *INFRARED_SENSOR.as_mut_ptr() };
        *infrared_sensor = gpiob.pb14.into_pull_up_input(&mut gpiob.crh);
        // 配置 AFIO 外部中断引脚选择
        infrared_sensor.make_interrupt_source(&mut afio);
        // 从该引脚启用外部中断
        infrared_sensor.enable_interrupt(&mut exti);
        // 在上升沿生成中断
        infrared_sensor.trigger_on_edge(&mut exti, Edge::Rising);
    }

    oled.show_string(1, 1, "Count:");
    loop {
        oled.show_num(1, 7, get_sensor_count(), 5);
    }
}

#[interrupt]
fn EXTI15_10() {
    let infrared_sensor = unsafe { &mut *INFRARED_SENSOR.as_mut_ptr() };

    if infrared_sensor.check_interrupt() {
        unsafe {
            SENSOR_COUNT += 1;
        }

        // if we don't clear this bit, the ISR would trigger indefinitely
        infrared_sensor.clear_interrupt_pending_bit();
    }
}

/// 获取传感器计数
fn get_sensor_count() -> u32 {
    unsafe { SENSOR_COUNT }
}

/// 初始化外设
fn init_peripheral() -> (
    flash::Parts,
    rcc::Rcc,
    cortex_m::peripheral::SYST,
    afio::Parts,
    pac::EXTI,
    gpioa::Parts,
    gpiob::Parts,
) {
    // 获取对外设的访问对象
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();
    let flash: flash::Parts = dp.FLASH.constrain();
    let rcc: rcc::Rcc = dp.RCC.constrain();
    let system_timer = cp.SYST;
    let afio: stm32f1xx_hal::afio::Parts = dp.AFIO.constrain();
    let exti: pac::EXTI = dp.EXTI;
    let mut nvic = cp.NVIC;
    unsafe {
        // Enable EXTI15_10 interruptions
        NVIC::unmask(interrupt::EXTI15_10);
        // 将中断的“优先级”设置为prio
        nvic.set_priority(interrupt::EXTI15_10, 0x80);
    }

    let gpioa: gpioa::Parts = dp.GPIOA.split();
    let gpiob: gpiob::Parts = dp.GPIOB.split();
    (flash, rcc, system_timer, afio, exti, gpioa, gpiob)
}

/// 封装具有自定义精度的阻塞延迟函数
fn sys_delay(
    mut flash: flash::Parts,
    rcc: rcc::Rcc,
    system_timer: cortex_m::peripheral::SYST,
) -> SysDelay {
    // 冻结系统中所有时钟的配置，并将冻结的频率存储在时钟中
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    // 具有自定义精度的阻塞延迟
    system_timer.delay(&clocks)
}
