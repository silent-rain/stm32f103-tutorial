#![no_std]
#![no_main]
#![allow(clippy::empty_loop)]

use core::cell::RefCell;

use hardware::oled;

use defmt::println;
use defmt_rtt as _;
use panic_probe as _;

use cortex_m::interrupt::Mutex;
use cortex_m::peripheral::NVIC;
use cortex_m_rt::entry;
use stm32f1xx_hal::device::TIM2;
use stm32f1xx_hal::pac;
use stm32f1xx_hal::pac::interrupt;
use stm32f1xx_hal::prelude::_fugit_ExtU32;
use stm32f1xx_hal::prelude::_fugit_RateExtU32;
use stm32f1xx_hal::prelude::_stm32_hal_flash_FlashExt;
use stm32f1xx_hal::prelude::_stm32_hal_gpio_GpioExt;
use stm32f1xx_hal::rcc::RccExt;
use stm32f1xx_hal::timer::CounterMs;
use stm32f1xx_hal::timer::{Event, TimerExt};

static G_TIM: Mutex<RefCell<Option<CounterMs<TIM2>>>> = Mutex::new(RefCell::new(None));

// 计数器
static mut NUM: u32 = 0;
static mut COUNT: u32 = 0;

#[entry]
fn main() -> ! {
    // 获取对外设的访问对象
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();
    let mut nvic = cp.NVIC;
    let tim2 = dp.TIM2;

    let mut gpiob = dp.GPIOB.split();

    // 冻结系统中所有时钟的配置，并将冻结的频率存储在时钟中
    let clocks = rcc
        .cfgr
        .use_hse(10.kHz())
        .sysclk(10.kHz())
        .freeze(&mut flash.acr);

    unsafe {
        // Enable interruptions
        NVIC::unmask(interrupt::TIM2);
        // 设置中断的优先级
        nvic.set_priority(interrupt::TIM2, 2);
    }

    // 配置系统定时器，每秒触发一次更新，并启用中断
    let mut timer = tim2.counter_ms(&clocks);

    // 设置一个1秒后过期的计时器
    // 定时时间计算公式：CK_CNT_OV = CK_CNT/(ARR+1) = CK_PSC/(PSC+1)/(ARR+1)
    // 定时 1s 即定时频率为 1Hz；
    timer.start(1.secs()).unwrap();

    // 当计时器到期时产生中断
    timer.listen(Event::Update);

    // 将计时器移动到全局存储中
    cortex_m::interrupt::free(|cs| *G_TIM.borrow(cs).borrow_mut() = Some(timer));

    // 初始化 OLED 显示屏
    println!("load oled...");
    let mut oled = oled::simple::init_oled(gpiob.pb8, gpiob.pb9, &mut gpiob.crh);

    oled.show_string(1, 1, "Num:");
    oled.show_string(2, 1, "Cnt:");
    loop {
        oled.show_num(1, 5, get_count(), 5);
        oled.show_num(2, 5, get_num(), 5);
    }
}

/// 中断调用函数
#[interrupt]
fn TIM2() {
    static mut TIM: Option<CounterMs<TIM2>> = None;
    let tim = TIM.get_or_insert_with(|| {
        cortex_m::interrupt::free(|cs| G_TIM.borrow(cs).replace(None).unwrap())
    });

    // if tim.get_interrupt() == Event::Update {
    //     unsafe {
    //         NUM += 1;
    //     }
    // }
    unsafe {
        NUM += 1;
        COUNT = tim.now().ticks();
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
