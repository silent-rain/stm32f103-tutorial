#![no_std]
#![no_main]
#![allow(clippy::empty_loop)]

use hardware::oled;

use defmt::println;
use defmt_rtt as _;
use panic_probe as _;

use cortex_m::prelude::_embedded_hal_blocking_delay_DelayMs;
use cortex_m_rt::entry;
use stm32f1xx_hal::gpio;
use stm32f1xx_hal::pac;
use stm32f1xx_hal::pac::TIM2;
use stm32f1xx_hal::prelude::{
    _fugit_RateExtU32, _stm32_hal_flash_FlashExt, _stm32_hal_gpio_GpioExt,
};
use stm32f1xx_hal::rcc::RccExt;
use stm32f1xx_hal::timer::DelayMs;
use stm32f1xx_hal::timer::TimerExt;

#[entry]
fn main() -> ! {
    // 获取对外设的访问对象
    let _cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC;
    let wwdg = &dp.WWDG;

    let mut gpiob = dp.GPIOB.split();

    // 启用窗口看门狗时钟
    rcc.apb1enr.modify(|_, w| w.wwdgen().enabled());

    // 检查WWDG复位标志位
    if rcc.csr.read().wwdgrstf().is_reset() {
        println!("WWDGRST..");

        // 清除复位标志
        rcc.csr.modify(|_, w| w.wwdgrstf().clear_bit());
    } else {
        println!("RST..");
    }

    // 冻结系统中所有时钟的配置，并将冻结的频率存储在时钟中
    let clocks = rcc.constrain().cfgr.pclk1(36.MHz()).freeze(&mut flash.acr);

    // 具有自定义精度的阻塞延迟函数
    let mut delay = dp.TIM2.delay_ms(&clocks);

    // 初始化 OLED 显示屏
    println!("load oled...");
    let mut oled = oled::simple::init_oled(gpiob.pb8, gpiob.pb9, &mut gpiob.crh);

    // 按键
    let mut key = gpiob.pb1.into_pull_up_input(&mut gpiob.crl);

    oled.show_string(1, 1, "WWDG TEST");
    delay.delay_ms(1000_u32);

    // 设置窗口看门狗的预分频值和窗口值
    wwdg.cfr.modify(|_, w| {
        // 设置预分频值
        w.wdgtb()
            .div8()
            // 设置窗口值
            .w()
            .bits(0x40 + 21) // 30ms
    });

    // 使能WWDG并设置初始计数值为 0x40 + 54
    wwdg.cr
        .modify(|_, w| w.wdga().enabled().t().bits(0x40 + 54)); // 50ms

    println!("loop..");
    loop {
        // 按键事件
        // 按住按键不放，模拟程序卡死晚喂狗的情况
        get_key_status(&mut key, &mut delay);

        // oled 显示比较耗时
        // oled.show_string(3, 1, "FEED");
        // delay.delay_ms(20_u32);
        // oled.show_string(3, 1, "    ");
        // delay.delay_ms(20_u32);

        // 使用 SysDelay 来实现延时功能，那么你可能会遇到 WWDG 复位的问题。
        // 因为 SysDelay 会在延时期间关闭中断，导致 WWDG 中断无法执行喂狗的操作。
        // delay.delay_ms(30_u32); // 过快喂狗
        // delay.delay_ms(50_u32); // 超时喂狗
        // delay.delay_ms(40_u32); // 正常喂狗

        // delay.delay_ms(20_u32);
        // println!("delay1..");
        // delay.delay_ms(10_u32);
        // println!("delay2..");
        // delay.delay_ms(2_u32);
        // println!("delay3..");

        // 模拟耗时
        for i in 0..240 {
            println!("code..{:?}", i);
        }

        // 喂狗
        // 30ms-50ms
        wwdg.cr.modify(|_, w| w.t().bits(0x40 + 54));
    }
}

/// 获取按键的状态
/// 按键是否按下
fn get_key_status(
    key1: &mut gpio::Pin<'B', 1, gpio::Input<gpio::PullUp>>,
    delay: &mut DelayMs<TIM2>,
) -> bool {
    let mut key_num = false;

    if key1.is_low() {
        // 按键按下抖动
        delay.delay_ms(20_u32);
        // 按着不动, 松手后跳出循环
        while key1.is_low() {}
        // 按键松开抖动
        delay.delay_ms(20_u32);

        key_num = true;
    }
    key_num
}
