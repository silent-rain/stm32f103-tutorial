#![allow(clippy::empty_loop)]
#![no_std]
#![no_main]

use core::cell::RefCell;

mod hardware;
use cortex_m::prelude::_embedded_hal_Qei;
use hardware::oled;
use hardware::peripheral::Peripheral;

use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

use cortex_m::interrupt::Mutex;
use cortex_m::peripheral::NVIC;
use cortex_m_rt::entry;
use stm32f1xx_hal::device::tim2::smcr::ETP_A;
use stm32f1xx_hal::device::TIM2;
use stm32f1xx_hal::gpio::ExtiPin;
use stm32f1xx_hal::pac::interrupt;
use stm32f1xx_hal::prelude::_fugit_ExtU32;
use stm32f1xx_hal::prelude::_fugit_RateExtU32;
use stm32f1xx_hal::qei::QeiOptions;
use stm32f1xx_hal::rcc::BusClock;
use stm32f1xx_hal::timer::Channel;
use stm32f1xx_hal::timer::Configuration;
use stm32f1xx_hal::timer::CounterHz;
use stm32f1xx_hal::timer::CounterMs;
use stm32f1xx_hal::timer::PwmExt;
use stm32f1xx_hal::timer::ReadMode;
use stm32f1xx_hal::timer::Remap;
use stm32f1xx_hal::timer::Tim2NoRemap;
use stm32f1xx_hal::timer::Timer;
use stm32f1xx_hal::timer::{Event, TimerExt};

static G_TIM: Mutex<RefCell<Option<CounterHz<TIM2>>>> = Mutex::new(RefCell::new(None));

// 计数器
static mut NUM: u32 = 0;
static mut COUNT: u32 = 0;
static mut COUNT2: u32 = 0;
static mut ARR: u32 = 0;

#[entry]
fn main() -> ! {
    rtt_init_print!();

    // 初始化外设
    let Peripheral {
        mut flash,
        rcc,
        tim2,
        syst: _,
        mut afio,
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
        .use_hse(8.MHz())
        .sysclk(72.MHz())
        .pclk1(36.MHz())
        .freeze(&mut flash.acr);

    unsafe {
        // Enable interruptions
        NVIC::unmask(interrupt::TIM2);
        // 设置中断的优先级
        nvic.set_priority(interrupt::TIM2, 2);
    }

    // let mut pa0 = gpioa.pa0.into_pull_up_input(&mut gpioa.crl);
    // pa0.make_interrupt_source(&mut afio);

    // Configure TIM2 ETR input
    // tim2.smcr.write(|w| {
    //     w.etps()
    //         .bits(0b00) // Set external trigger prescaler to OFF
    //         .etp()
    //         .clear_bit() // Set external trigger polarity to non-inverted
    //         .ece()
    //         .set_bit() // Enable external clock mode 2
    //         .etf()
    //         .bits(0x0F) // Set external trigger filter to 0x0F
    // });

    // 配置系统定时器
    let mut timer: CounterHz<TIM2> = tim2.counter_hz(&clocks);

    // 在倒计时模式下重新启动定时器，使用用户定义的预调度器和自动重新加载寄存器
    // timer.start_raw(1000, 1000);
    // timer.start(1.Hz()).unwrap();

    // 当计时器到期时产生中断
    timer.listen(Event::Update);

    // 将计时器移动到全局存储中
    cortex_m::interrupt::free(|cs| *G_TIM.borrow(cs).borrow_mut() = Some(timer));

    // 初始化 OLED 显示屏
    rprintln!("load oled...");
    let (mut scl, mut sda) = oled::init_oled(gpiob.pb8, gpiob.pb9, &mut gpiob.crh);

    oled::show_string(&mut scl, &mut sda, 1, 1, "Num:");
    oled::show_string(&mut scl, &mut sda, 2, 1, "Arr:");
    oled::show_string(&mut scl, &mut sda, 3, 1, "Cnt:");
    loop {
        oled::show_num(&mut scl, &mut sda, 1, 5, get_num(), 5);
        oled::show_num(&mut scl, &mut sda, 2, 5, get_arr(), 5);
        oled::show_num(&mut scl, &mut sda, 3, 5, get_count(), 5);
        oled::show_num(&mut scl, &mut sda, 4, 5, get_count2(), 5);
    }
}

/// 中断调用函数
#[interrupt]
fn TIM2() {
    static mut TIM: Option<CounterHz<TIM2>> = None;
    let tim = TIM.get_or_insert_with(|| {
        cortex_m::interrupt::free(|cs| G_TIM.borrow(cs).replace(None).unwrap())
    });

    unsafe {
        NUM += 1;
        // 检索自动重新加载寄存器的值。
        ARR = tim.arr() as u32;
        // 检索预计算器寄存器的内容。真正的预标量是这个值+ 1。
        // COUNT2 = tim.psc() as u32;
        // 从Duration中提取刻度。
        COUNT = tim.now().ticks();
        // 返回当前存储的标志的原始值。
        COUNT2 = tim.get_interrupt().bits();
    }
    tim.wait().unwrap();
}

/// 获取计数
fn get_num() -> u32 {
    unsafe { NUM }
}

/// 获取计数
fn get_count() -> u32 {
    unsafe { COUNT }
}

/// 获取计数
fn get_count2() -> u32 {
    unsafe { COUNT2 }
}

/// 获取计数
fn get_arr() -> u32 {
    unsafe { ARR }
}
