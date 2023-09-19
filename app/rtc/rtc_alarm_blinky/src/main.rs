#![no_std]
#![no_main]
#![allow(clippy::empty_loop)]

use defmt::println;
use defmt_rtt as _;
use panic_probe as _;

use cortex_m_rt::entry;
use nb::block;
use stm32f1xx_hal::pac;
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

    let mut gpioa = dp.GPIOA.split();

    // 冻结系统中所有时钟的配置，并将冻结的频率存储在时钟中
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    // 具有自定义精度的阻塞延迟函数
    let mut _delay = syst.delay(&clocks);

    // 初始化 LED
    let mut led = gpioa.pa0.into_push_pull_output(&mut gpioa.crl);

    // 设置RTC
    // 启用对备份域的写入
    let mut backup_domain = rcc.bkp.constrain(dp.BKP, &mut pwr);
    // 启动RTC
    let mut rtc = Rtc::new(dp.RTC, &mut backup_domain);

    let mut led_on = false;

    println!("loop");
    loop {
        // 将当前时间设置为0
        rtc.set_time(0);
        // 在 5 秒内触发 LED 闪烁
        rtc.set_alarm(5);
        block!(rtc.wait_alarm()).unwrap();
        if led_on {
            led.set_low();
            led_on = false;
        } else {
            led.set_high();
            led_on = true;
        }
    }
}
