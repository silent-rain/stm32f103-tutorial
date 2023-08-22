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
use cortex_m::peripheral::NVIC;
use cortex_m_rt::entry;
use stm32f1xx_hal::device::TIM2;
use stm32f1xx_hal::gpio::ExtiPin;
use stm32f1xx_hal::pac::interrupt;
use stm32f1xx_hal::prelude::_fugit_RateExtU32;
use stm32f1xx_hal::rcc::Clocks;
use stm32f1xx_hal::timer::CounterHz;
use stm32f1xx_hal::timer::{Event, TimerExt};

static G_TIM: Mutex<RefCell<Option<CounterHz<TIM2>>>> = Mutex::new(RefCell::new(None));

// 计数器
static mut NUM: u32 = 0;
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
        // // 设置系统时钟
        // .sysclk(72.MHz())
        // .pclk1(36.MHz())
        .freeze(&mut flash.acr);

    let mut pa0 = gpioa.pa0.into_pull_up_input(&mut gpioa.crl);
    // pa0.make_interrupt_source(&mut afio);

    // Configure the external clock mode 2 on TIM2
    // tim2.smcr.modify(|_, w| {
    //     // Enable external clock mode 2
    //     w.ece().set_bit();
    //     // External trigger prescaler
    //     w.etps().bits(0b00);
    //     // w.etps().div1();
    //     // Set external trigger polarity to non-inverted
    //     w.etp().clear_bit();
    //     // Set external trigger filter to 0x0F
    //     // w.etf().bits(0b0000);
    //     w.etf().no_filter();
    //     w.msm().set_bit();
    //     w.ts().itr0();
    //     w.sms().encoder_mode_2()
    // });

    // Clear the update flag before enabling interrupts
    // tim2.sr.modify(|_, w| w.uif().clear_bit());

    // // Enable update interrupts
    // tim2.dier.modify(|_, w| w.uie().set_bit());
    rprintln!("timer...");

    // 配置系统定时器
    let mut timer: CounterHz<TIM2> = tim2.counter_hz(&clocks);

    // 在倒计时模式下重新启动定时器，使用用户定义的预调度器和自动重新加载寄存器
    timer.start_raw(1000, 1000);
    // timer.start(1.Hz()).unwrap();
    // timer.start(clocks.pclk1_tim()).unwrap();
    // timer.reset();

    // 当计时器到期时产生中断
    timer.listen(Event::Update);

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
    oled::show_string(&mut scl, &mut sda, 2, 1, "Arr:");
    oled::show_string(&mut scl, &mut sda, 3, 1, "Cnt:");
    loop {
        oled::show_num(&mut scl, &mut sda, 1, 5, get_num(), 5);
        oled::show_num(&mut scl, &mut sda, 2, 5, get_arr(), 5);
        oled::show_num(&mut scl, &mut sda, 3, 5, get_counter(), 5);
    }
}

/// 中断调用函数
#[interrupt]
fn TIM2() {
    cortex_m::interrupt::free(|cs| {
        if let Some(tim) = G_TIM.borrow(cs).borrow_mut().as_mut() {
            unsafe {
                NUM += 1;
                ARR = tim.arr() as u32;
            }
            rprintln!("TIM2...");
            tim.wait().unwrap();
        }
    });
}
/// 获取计数
fn get_num() -> u32 {
    unsafe { NUM }
}

/// 获取自动重新加载寄存器
fn get_arr() -> u32 {
    unsafe { ARR }
}

/// 获取计数
fn get_counter() -> u32 {
    let mut count = 0;
    cortex_m::interrupt::free(|cs| {
        if let Some(tim) = G_TIM.borrow(cs).borrow_mut().as_mut() {
            count = tim.now().ticks();
        }
    });
    count
}
