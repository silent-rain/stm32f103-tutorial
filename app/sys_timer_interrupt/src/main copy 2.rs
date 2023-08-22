#![allow(clippy::empty_loop)]
#![no_std]
#![no_main]

use core::cell::RefCell;

mod hardware;
use cortex_m::peripheral::syst::SystClkSource;
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
use stm32f1xx_hal::timer::SysEvent;
use stm32f1xx_hal::timer::Tim2NoRemap;
use stm32f1xx_hal::timer::Tim2PartialRemap2;
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
        mut syst,
        mut afio,
        exti: _,
        mut dbg,
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
        // .sysclk(72.MHz())
        // .pclk1(36.MHz())
        .freeze(&mut flash.acr);

    // unsafe {
    //     // Enable interruptions
    //     NVIC::unmask(interrupt::TIM2);
    //     // 设置中断的优先级
    //     nvic.set_priority(interrupt::TIM2, 2);
    // }

    let pa0 = gpioa.pa0;
    let pa1 = gpioa.pa1;

    let pwm_input = Timer::new(tim2, &clocks).pwm_input::<Tim2NoRemap, (
        stm32f1xx_hal::gpio::Pin<'A', 0>,
        stm32f1xx_hal::gpio::Pin<'A', 1>,
    )>(
        (pa0, pa1),
        &mut afio.mapr,
        &mut dbg,
        Configuration::Frequency(1.Hz()),
    );

    // let mut pa0 = gpioa.pa0.into_pull_up_input(&mut gpioa.crl);
    // pa0.make_interrupt_source(&mut afio);

    // 外部参考时钟
    // syst.set_clock_source(SystClkSource::External);
    // syst.set_reload(12_000_000);
    // syst.clear_current();
    // syst.enable_counter();
    // syst.enable_interrupt();

    // let mut timer: stm32f1xx_hal::timer::CounterHz<TIM2> = Timer::syst(syst, &clocks).counter_hz();
    // timer.listen(SysEvent::Update);
    // timer.start(1.Hz()).unwrap();

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

    // // 配置系统定时器
    // let mut timer: CounterHz<TIM2> = tim2.counter_hz(&clocks);

    // // 在倒计时模式下重新启动定时器，使用用户定义的预调度器和自动重新加载寄存器
    // // timer.start_raw(1000, 1000);
    // // timer.start(1.Hz()).unwrap();

    // // 当计时器到期时产生中断
    // timer.listen(Event::Update);

    // // 将计时器移动到全局存储中
    // cortex_m::interrupt::free(|cs| *G_TIM.borrow(cs).borrow_mut() = Some(timer));

    // 初始化 OLED 显示屏
    rprintln!("load oled...");
    let (mut scl, mut sda) = oled::init_oled(gpiob.pb8, gpiob.pb9, &mut gpiob.crh);

    oled::show_string(&mut scl, &mut sda, 1, 1, "Num:");
    oled::show_string(&mut scl, &mut sda, 2, 1, "Arr:");
    oled::show_string(&mut scl, &mut sda, 3, 1, "Cnt:");
    loop {
        let freq = pwm_input
            .read_frequency(ReadMode::WaitForNextCapture, &clocks)
            .unwrap();
        let duty_cycle = pwm_input.read_duty(ReadMode::WaitForNextCapture).unwrap();
        oled::show_num(&mut scl, &mut sda, 1, 5, get_num(), 5);
        oled::show_num(&mut scl, &mut sda, 2, 5, freq.raw(), 5);
        // oled::show_num(&mut scl, &mut sda, 3, 5,  duty_cycle.1.into(), 5);
        oled::show_num(&mut scl, &mut sda, 4, 5, duty_cycle.0.into(), 5);
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
