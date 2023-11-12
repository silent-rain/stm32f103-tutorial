#![no_std]
#![no_main]
#![allow(clippy::empty_loop)]

use hardware::oled;

use defmt::println;
use defmt_rtt as _;
use panic_probe as _;

use cortex_m::peripheral::syst::SystClkSource;
use cortex_m::peripheral::NVIC;
use cortex_m_rt::{entry, exception};
use stm32f1xx_hal::pac::{self, interrupt};
use stm32f1xx_hal::prelude::{
    _fugit_RateExtU32, _stm32_hal_flash_FlashExt, _stm32_hal_gpio_GpioExt,
};
use stm32f1xx_hal::rcc::RccExt;
use stm32f1xx_hal::timer::{SysCounterHz, SysEvent, Timer};

// 计数器
static mut NUM: u32 = 0;

#[entry]
fn main() -> ! {
    // 获取对外设的访问对象
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();
    let mut syst = cp.SYST;
    let mut nvic = cp.NVIC;

    let mut gpiob = dp.GPIOB.split();

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

    unsafe {
        // Enable interruptions
        NVIC::unmask(interrupt::TIM2);
        // 设置中断的优先级
        nvic.set_priority(interrupt::TIM2, 2);
        // NVIC::unpend(interrupt::TIM2);
    }

    // 初始化 OLED 显示屏
    println!("load oled...");
    let mut oled = oled::simple::init_oled(gpiob.pb8, gpiob.pb9, &mut gpiob.crh);

    oled.show_string(1, 1, "Num:");
    loop {
        oled.show_num(1, 5, get_num(), 5);
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
