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
use stm32f1xx_hal::prelude::_fugit_ExtU32;
use stm32f1xx_hal::prelude::_stm32_hal_flash_FlashExt;
use stm32f1xx_hal::prelude::_stm32_hal_gpio_GpioExt;
use stm32f1xx_hal::rcc::RccExt;
use stm32f1xx_hal::timer::SysDelay;
use stm32f1xx_hal::timer::SysTimerExt;
use stm32f1xx_hal::watchdog;

#[entry]
fn main() -> ! {
    // 获取对外设的访问对象
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();
    let syst = cp.SYST;
    let iwdg = dp.IWDG;

    let mut gpiob = dp.GPIOB.split();

    // 冻结系统中所有时钟的配置，并将冻结的频率存储在时钟中
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    // 具有自定义精度的阻塞延迟函数
    let mut delay = syst.delay(&clocks);

    // 初始化 OLED 显示屏
    println!("load oled...");
    let mut oled = oled::simple::init_oled(gpiob.pb8, gpiob.pb9, &mut gpiob.crh);

    // 按键
    let mut key = gpiob.pb1.into_pull_up_input(&mut gpiob.crl);

    // 检查是否由于IWDG复位
    let rcc_b = unsafe { &*RCC::ptr() };
    if rcc_b.csr.read().iwdgrstf().is_reset() {
        oled.show_string(2, 1, "IWDGRST");
        delay.delay_ms(1000_u16);
        // oled.show_string(2, 1, "       ");
        delay.delay_ms(100_u16);

        rcc_b.csr.modify(|_, w| w.iwdgrstf().clear_bit());
    } else {
        oled.show_string(2, 1, "RST");
        delay.delay_ms(500_u16);
        // oled.show_string(2, 1, "   ");
        delay.delay_ms(100_u16);
    }

    oled.show_string(1, 1, "IWDG TEST");

    let mut watchdog = watchdog::IndependentWatchdog::new(iwdg);

    // 以等于5秒的重新加载值启动IWDG
    watchdog.start(5.secs());

    loop {
        // 按键事件
        // 按住按键不放，模拟程序卡死的情况
        get_key_status(&mut key, &mut delay);

        // Feed the IWDG to prevent a reset
        // 开始喂狗，间隔时间不能超过上面的 5s
        watchdog.feed();

        oled.show_string(3, 1, "FEED");
        delay.delay_ms(200_u32);
        oled.show_string(3, 1, "    ");
        delay.delay_ms(600_u32);
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
