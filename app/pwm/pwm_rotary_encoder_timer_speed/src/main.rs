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
use cortex_m::prelude::_embedded_hal_Qei;
use cortex_m_rt::entry;
use stm32f1xx_hal::gpio::Input;
use stm32f1xx_hal::gpio::Pin;
use stm32f1xx_hal::gpio::PullUp;
use stm32f1xx_hal::pac;
use stm32f1xx_hal::pac::{interrupt, TIM2, TIM3};
use stm32f1xx_hal::prelude::{
    _fugit_ExtU32, _stm32_hal_afio_AfioExt, _stm32_hal_flash_FlashExt, _stm32_hal_gpio_GpioExt,
};
use stm32f1xx_hal::qei::Qei;
use stm32f1xx_hal::qei::QeiOptions;
use stm32f1xx_hal::rcc::RccExt;
use stm32f1xx_hal::timer::CounterMs;
use stm32f1xx_hal::timer::Event;
use stm32f1xx_hal::timer::SysTimerExt;
use stm32f1xx_hal::timer::Tim3NoRemap;
use stm32f1xx_hal::timer::Timer;
use stm32f1xx_hal::timer::TimerExt;

type TQei = Qei<TIM3, Tim3NoRemap, (Pin<'A', 6, Input<PullUp>>, Pin<'A', 7, Input<PullUp>>)>;

static mut BEFORE_COUNT: u16 = 0;
static mut SPEED: i16 = 0;

static G_TIM: Mutex<RefCell<Option<CounterMs<TIM2>>>> = Mutex::new(RefCell::new(None));
static G_QEI: Mutex<RefCell<Option<TQei>>> = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    // 获取对外设的访问对象
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();
    let syst = cp.SYST;
    let mut afio = dp.AFIO.constrain();
    let mut nvic = cp.NVIC;
    let tim2 = dp.TIM2;
    let tim3 = dp.TIM3;

    let mut gpioa = dp.GPIOA.split();
    let mut gpiob = dp.GPIOB.split();

    // 冻结系统中所有时钟的配置，并将冻结的频率存储在时钟中
    let clocks = rcc
        .cfgr
        // .use_hse(8.MHz())
        // 设置系统时钟
        // .sysclk(72.MHz())
        // .pclk1(36.MHz())
        // .hclk(72.MHz())
        .freeze(&mut flash.acr);

    // 具有自定义精度的阻塞延迟函数
    let mut _delay = syst.delay(&clocks);

    // 初始化 OLED 显示屏
    println!("load oled...");
    let mut oled = oled::simple::init_oled(gpiob.pb8, gpiob.pb9, &mut gpiob.crh);

    println!("load timer...");
    let mut timer = tim2.counter_ms(&clocks);
    timer.start(1.secs()).unwrap();
    timer.listen(Event::Update);
    unsafe {
        NVIC::unmask(interrupt::TIM2);
        nvic.set_priority(interrupt::TIM2, 2);
    }

    // 旋转编码器
    // 配置上拉输入
    println!("load rotary encoder ...");
    let pa6 = gpioa.pa6.into_pull_up_input(&mut gpioa.crl);
    let pa7 = gpioa.pa7.into_pull_up_input(&mut gpioa.crl);
    let qei = Timer::new(tim3, &clocks).qei((pa6, pa7), &mut afio.mapr, QeiOptions::default());

    // 移动到全局存储中
    cortex_m::interrupt::free(|cs| *G_TIM.borrow(cs).borrow_mut() = Some(timer));
    cortex_m::interrupt::free(|cs| G_QEI.borrow(cs).replace(Some(qei)));

    oled.show_string(1, 1, "Cnt:");
    oled.show_string(2, 1, "Speed:");
    println!("loop ...");
    loop {
        // 获取当前编码器计数
        let count = get_count();
        let speed = get_speed();
        println!("loop cnt={:?} speed={:?}", count, speed);
        oled.show_signed_num(1, 5, count as i32, 5);
        oled.show_signed_num(2, 7, speed as i32, 5);
    }
}

/// 获取当前编码器计数
fn get_count() -> i16 {
    let mut count = 0;
    cortex_m::interrupt::free(|cs| {
        if let Some(qei) = G_QEI.borrow(cs).borrow_mut().as_mut() {
            count = qei.count();
        }
    });
    count as i16
}

/// 获取当前编码器计数
fn get_speed() -> i16 {
    unsafe { SPEED }
}

/// 中断调用函数
#[interrupt]
fn TIM2() {
    cortex_m::interrupt::free(|cs| {
        if let Some(tim) = G_TIM.borrow(cs).borrow_mut().as_mut() {
            tim.wait().unwrap();

            if let Some(qei) = G_QEI.borrow(cs).borrow_mut().as_mut() {
                unsafe {
                    let before = BEFORE_COUNT;
                    let after = qei.count();

                    BEFORE_COUNT = after; // 重置计数

                    SPEED = after.wrapping_sub(before) as i16;
                    println!("speed={:?}", SPEED);
                }
            }
        }
    });
}
