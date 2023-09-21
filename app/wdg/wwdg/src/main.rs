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
use stm32f1xx_hal::pac::RCC;
use stm32f1xx_hal::prelude::_stm32_hal_flash_FlashExt;
use stm32f1xx_hal::prelude::_stm32_hal_gpio_GpioExt;
use stm32f1xx_hal::rcc::RccExt;
use stm32f1xx_hal::timer::SysDelay;
use stm32f1xx_hal::timer::SysTimerExt;

#[entry]
fn main() -> ! {
    // 获取对外设的访问对象
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();
    let syst = cp.SYST;
    let wwdg = dp.WWDG;

    let mut gpiob = dp.GPIOB.split();

    // 冻结系统中所有时钟的配置，并将冻结的频率存储在时钟中
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    // 具有自定义精度的阻塞延迟函数
    let mut delay = syst.delay(&clocks);

    // 初始化 OLED 显示屏
    println!("load oled...");
    let (mut scl, mut sda) = oled::simple::init_oled_pin(gpiob.pb8, gpiob.pb9, &mut gpiob.crh);
    let mut oled = oled::OLED::new(&mut scl, &mut sda);

    // 按键
    let mut key = gpiob.pb1.into_pull_up_input(&mut gpiob.crl);

    oled.show_string(1, 1, "WWDG TEST");

    let rcc_b = unsafe { &*RCC::ptr() };
    // 检查是否由窗口看门狗复位
    if rcc_b.csr.read().wwdgrstf().is_reset() {
        oled.show_string(2, 1, "WWDGRST");
        // delay.delay_ms(500_u16);
        // oled.show_string( 2, 1, "       ");
        // delay.delay_ms(100_u16);

        // 清除复位标志
        rcc_b.csr.modify(|_, w| w.wwdgrstf().clear_bit());
    } else {
        oled.show_string(2, 1, "RST");
        // delay.delay_ms(500_u16);
        // oled.show_string( 2, 1, "   ");
        // delay.delay_ms(100_u16);
    }

    // 启用窗口看门狗时钟
    rcc_b.apb1enr.modify(|_, w| w.wwdgen().enabled());

    // 设置窗口看门狗的预分频值和窗口值
    wwdg.cfr.modify(|_, w| {
        // 设置预分频值
        w.wdgtb()
            // .bits(0b00)
            .div8()
            // 设置窗口值
            // 当窗口看门狗的计数器的值在这个窗口值以下时，你可以安全地"喂狗"（也就是重置计数器）。
            // 如果你在计数器的值大于这个窗口值时尝试"喂狗"，系统将会立即重置。
            .w()
            .bits(0x40 + 2)
    });

    // 启动窗口看门狗
    // 这是窗口看门狗计数器的初始值。在这个例子中，我们将其设置为最大值 0x40 + 54。
    // 这意味着窗口看门狗在超时并重置系统之前，计数器将从 0x40 + 54 倒数到0。
    wwdg.cr
        .modify(|_, w| w.wdga().enabled().t().bits(0x40 + 54));

    loop {
        // 按键事件
        // 按住按键不放，模拟程序卡死晚喂狗的情况
        get_key_status(&mut key, &mut delay);

        oled.show_string(3, 1, "FEED");
        delay.delay_ms(20_u32);
        oled.show_string(3, 1, "    ");
        delay.delay_ms(5000_u32);

        // 喂狗
        // 5-10s
        println!("dog");
        wwdg.cr.modify(|_, w| w.t().bits(0x40 + 54));
    }
}

/// 获取按键的状态
/// 按键是否按下
fn get_key_status(
    key1: &mut gpio::Pin<'B', 1, gpio::Input<gpio::PullUp>>,
    delay: &mut SysDelay,
) -> bool {
    let mut key_num = false;

    if key1.is_low() {
        // 按键按下抖动
        delay.delay_ms(20_u16);
        // 按着不动, 松手后跳出循环
        while key1.is_low() {}
        // 按键松开抖动
        delay.delay_ms(20_u16);

        key_num = true;
    }
    key_num
}
