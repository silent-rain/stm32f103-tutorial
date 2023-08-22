#![allow(clippy::empty_loop)]
#![no_std]
#![no_main]

use core::cell::RefCell;

mod hardware;
use hardware::oled;
use hardware::peripheral::Peripheral;

use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

use cortex_m::interrupt::Mutex;
use cortex_m::peripheral::syst::SystClkSource;
use cortex_m::peripheral::NVIC;
use cortex_m_rt::{entry, exception};
use stm32f1xx_hal::pac::interrupt;
use stm32f1xx_hal::prelude::_fugit_RateExtU32;
use stm32f1xx_hal::timer::{SysCounterHz, SysEvent, Timer};
static G_TIM: Mutex<RefCell<Option<SysCounterHz>>> = Mutex::new(RefCell::new(None));

// 计数器
static mut NUM: u32 = 0;

#[entry]
fn main() -> ! {
    rtt_init_print!();

    // 初始化外设
    let Peripheral {
        mut flash,
        rcc,
        tim2: _,
        mut syst,
        afio: _,
        exti: _,
        dbg: _,
        mut nvic,
        mut gpioa,
        mut gpiob,
    } = Peripheral::new();

    // 冻结系统中所有时钟的配置，并将冻结的频率存储在时钟中
    let clocks = rcc
        .cfgr
        // 使用HSE(外部振荡器)代替HSI(内部RC振荡器)作为时钟源。
        // 如果外部振荡器未连接或启动失败，将导致挂起。
        // 指定的频率必须是外部振荡器的频率
        // .use_hse(8.MHz())
        // 设置系统时钟
        // .sysclk(72.MHz())
        // .pclk1(36.MHz())
        .freeze(&mut flash.acr);

    let _pa0 = gpioa.pa0.into_pull_up_input(&mut gpioa.crl);

    // configures the system timer to trigger a SysTick exception every second
    syst.set_clock_source(SystClkSource::External);
    // this is configured for the LM3S6965 which has a default CPU clock of 12 MHz
    syst.set_reload(12_000_000);
    syst.clear_current();
    // syst.enable_counter();
    syst.enable_interrupt();

    let mut timer: SysCounterHz = Timer::syst_external(syst, &clocks).counter_hz();
    timer.start(1.Hz()).unwrap();
    timer.listen(SysEvent::Update);

    // 将计时器移动到全局存储中
    cortex_m::interrupt::free(|cs| G_TIM.borrow(cs).replace(Some(timer)));

    unsafe {
        // Enable interruptions
        NVIC::unmask(interrupt::TIM2);
        // 设置中断的优先级
        nvic.set_priority(interrupt::TIM2, 2);
        // NVIC::unpend(interrupt::TIM2);
    }

    // 初始化 OLED 显示屏
    rprintln!("load oled...");
    let (mut scl, mut sda) = oled::init_oled(gpiob.pb8, gpiob.pb9, &mut gpiob.crh);

    oled::show_string(&mut scl, &mut sda, 1, 1, "Num:");
    loop {
        oled::show_num(&mut scl, &mut sda, 1, 5, get_num(), 5);
    }
}

#[exception]
fn SysTick() {
    unsafe {
        NUM += 1;
    }
}

/// 获取计数
fn get_num() -> u32 {
    unsafe { NUM }
}
