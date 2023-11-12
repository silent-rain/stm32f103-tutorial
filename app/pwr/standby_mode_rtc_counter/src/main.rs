#![no_std]
#![no_main]
#![allow(clippy::empty_loop)]

use hardware::oled;

use defmt::println;
use defmt_rtt as _;
use panic_probe as _;

use cortex_m::asm::wfi;
use cortex_m_rt::entry;
use nb::block;
use stm32f1xx_hal::pac;
use stm32f1xx_hal::prelude::_embedded_hal_blocking_delay_DelayMs;
use stm32f1xx_hal::prelude::_stm32_hal_flash_FlashExt;
use stm32f1xx_hal::prelude::_stm32_hal_gpio_GpioExt;
use stm32f1xx_hal::rcc::RccExt;
use stm32f1xx_hal::rtc::Rtc;
use stm32f1xx_hal::timer::SysTimerExt;

#[entry]
fn main() -> ! {
    // 获取对外设的访问对象
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();
    let syst = cp.SYST;
    let mut pwr = dp.PWR;
    let mut scb = cp.SCB;

    let mut gpiob = dp.GPIOB.split();

    // 冻结系统中所有时钟的配置，并将冻结的频率存储在时钟中
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    // 具有自定义精度的阻塞延迟函数
    let mut delay = syst.delay(&clocks);

    // 初始化 OLED 显示屏
    println!("load oled...");
    let mut oled = oled::simple::init_oled(gpiob.pb8, gpiob.pb9, &mut gpiob.crh);

    // 设置RTC
    // 启用对备份域的写入
    let mut backup_domain = rcc.bkp.constrain(dp.BKP, &mut pwr);
    // 启动RTC
    let mut rtc = Rtc::new(dp.RTC, &mut backup_domain);
    let alr_value = 10;
    loop {
        rtc.set_time(0);
        rtc.set_alarm(alr_value);
        block!(rtc.wait_alarm()).unwrap();
        rtc.set_time(0);

        oled.show_string(1, 1, "CNT:");
        oled.show_string(2, 1, "ALR:");
        // 获取不到该状态
        // oled.show_string( 3, 1, "ALRF:");

        let count = rtc.current_time();
        println!("current_time: {}", count);
        oled.show_num(1, 6, count, 10);
        oled.show_num(2, 6, alr_value, 10);

        oled.show_string(4, 1, "running");
        delay.delay_ms(100_u32);
        oled.show_string(4, 1, "       ");
        delay.delay_ms(100_u32);

        oled.show_string(4, 9, "STANDBY");
        delay.delay_ms(100_u32);
        oled.show_string(4, 9, "       ");
        delay.delay_ms(100_u32);

        oled.clear();

        // 当CPU进入深度睡眠时进入待机模式
        // 清除唤醒标识
        pwr.cr.modify(|_, w| w.cwuf().set_bit());
        // 进入待机模式
        pwr.cr.modify(|_, w| w.pdds().set_bit());
        // 设置SCR寄存器中的SLEEPDEEP位
        scb.set_sleepdeep();

        // 请求低功耗模式
        wfi();
    }
}
