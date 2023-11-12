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
use stm32f1xx_hal::prelude::_stm32_hal_flash_FlashExt;
use stm32f1xx_hal::prelude::_stm32_hal_gpio_GpioExt;
use stm32f1xx_hal::rcc::RccExt;
use stm32f1xx_hal::timer::Event;
use stm32f1xx_hal::timer::Timer;

// 计数器
static mut NUM: u32 = 0;

static G_TIM: Mutex<RefCell<Option<TIM2>>> = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    // 获取对外设的访问对象
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();
    let mut nvic = cp.NVIC;
    let tim2 = dp.TIM2;

    let mut gpioa = dp.GPIOA.split();
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

    // let _pa0 = gpioa.pa0.into_pull_up_input(&mut gpioa.crl);
    let _pa0 = gpioa.pa0.into_floating_input(&mut gpioa.crl);

    let mut cnt_timer = Timer::new(tim2, &clocks).counter_hz();
    // 使用用户定义的预分频器和自动重新加载寄存器以倒计时模式重新启动计时器
    cnt_timer.start_raw(0, 10);

    let tim2 = cnt_timer.release().release();

    // Configure the external clock mode 2 on TIM2
    tim2.smcr.modify(|_, w| {
        w
            // 外部时钟触发模式
            .sms()
            .encoder_mode_1()
            // 主/从模式
            .msm()
            .set_bit()
            // Enable external clock mode 2
            .ece()
            .enabled()
            // Set external trigger polarity to non-inverted
            .etp()
            .not_inverted()
            // External trigger prescaler
            .etps()
            .div1()
            // Set external trigger filter to 0x0F
            .etf()
            .no_filter()
            // 触发器选择, External Trigger input (ETRF)
            .ts()
            .ti1f_ed()
    });

    // 设置AFIO_MAPR寄存器的TIM2_REMAP位为00，选择PA0作为ETR引脚
    // afio.mapr
    //     .modify_mapr(|_, w| unsafe { w.tim2_remap().bits(0b00) });

    // CC1S=01，CC1为输入，IC1映射到TI1
    // tim2.ccmr1_input().write(|w| w.cc1s().ti1());
    // 启用捕获通道1
    // tim2.ccer.write(|w| w.cc1e().set_bit());

    // 配置定时器为从模式，使用 TIM2_ETR 引脚作为输入源，使用外部触发模式
    tim2.cr1.modify(|_, w| w.urs().set_bit().cen().set_bit());

    // 使能捕获中断
    // tim2.dier.write(|w| w.cc1ie().set_bit());
    // tim2.dier.write(|w| w.tie().set_bit());
    tim2.dier
        .modify(|r, w| unsafe { w.bits(r.bits() | Event::Update.bits()) });

    // 本质：在调用中断前，中断状态寄存器不能有标志位
    // 避免刚一上电就立刻进入中断，在 Time Init 的后面和中断的前面（手动清除中断标志位）
    tim2.sr.write(|w| w.cc1if().clear_bit());

    // 将计时器移动到全局存储中
    cortex_m::interrupt::free(|cs| G_TIM.borrow(cs).replace(Some(tim2)));

    unsafe {
        // Enable interruptions
        NVIC::unmask(interrupt::TIM2);
        // 设置中断的优先级
        nvic.set_priority(interrupt::TIM2, 1);
        // NVIC::unpend(interrupt::TIM2);
    }

    // 初始化 OLED 显示屏
    let mut oled = oled::simple::init_oled(gpiob.pb8, gpiob.pb9, &mut gpiob.crh);

    oled.show_string(1, 1, "Num:");
    oled.show_string(2, 1, "Arr:");
    oled.show_string(3, 1, "Psc:");
    oled.show_string(4, 1, "Cnt:");
    loop {
        oled.show_num(1, 5, get_num(), 5);
        oled.show_num(2, 5, get_arr(), 5);
        oled.show_num(3, 5, get_psc(), 5);
        oled.show_num(4, 5, get_counter(), 5);
    }
}

/// 中断调用函数
#[interrupt]
fn TIM2() {
    cortex_m::interrupt::free(|cs| {
        if let Some(tim2) = G_TIM.borrow(cs).borrow_mut().as_mut() {
            // 清除中断标志位
            if Event::from_bits_truncate(tim2.sr.read().bits()).contains(Event::Update) {
                tim2.sr
                    .write(|w| unsafe { w.bits(0xffff & !Event::Update.bits()) });
                unsafe {
                    NUM += 1;
                }
                println!("TIM2...");
            }
        }
    });
}

/// 获取计数
fn get_num() -> u32 {
    unsafe { NUM }
}

/// 获取自动重新加载寄存器
fn get_arr() -> u32 {
    let mut arr = 0;
    cortex_m::interrupt::free(|cs| {
        if let Some(tim2) = G_TIM.borrow(cs).borrow_mut().as_mut() {
            arr = tim2.arr.read().bits();
        }
    });
    arr
}

/// 获取预分频器（PSC）寄存器
fn get_psc() -> u32 {
    let mut psc = 0;
    cortex_m::interrupt::free(|cs| {
        if let Some(tim2) = G_TIM.borrow(cs).borrow_mut().as_mut() {
            psc = tim2.psc.read().bits();
        }
    });
    psc
}

/// 获取内部计数
fn get_counter() -> u32 {
    let mut count = 0;
    cortex_m::interrupt::free(|cs| {
        if let Some(tim2) = G_TIM.borrow(cs).borrow_mut().as_mut() {
            count = tim2.cnt.read().cnt().bits().into();
        }
    });
    count
}
