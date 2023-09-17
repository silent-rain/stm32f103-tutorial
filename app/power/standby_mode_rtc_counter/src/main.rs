#![allow(clippy::empty_loop)]
#![no_std]
#![no_main]

use hardware::oled;

use defmt::println;
use defmt_rtt as _;
use panic_probe as _;

use cortex_m_rt::entry;
use nb::block;
use stm32f1xx_hal::gpio;
use stm32f1xx_hal::gpio::OutputSpeed;
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

    let mut gpiob = dp.GPIOB.split();

    // 冻结系统中所有时钟的配置，并将冻结的频率存储在时钟中
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    // 具有自定义精度的阻塞延迟函数
    let mut delay = syst.delay(&clocks);

    // 初始化 OLED 显示屏
    println!("load oled...");
    let (mut scl, mut sda) = init_oled(gpiob.pb8, gpiob.pb9, &mut gpiob.crh);

    // 设置RTC
    // 启用对备份域的写入
    let mut backup_domain = rcc.bkp.constrain(dp.BKP, &mut pwr);
    // 启动RTC
    let mut rtc = Rtc::new(dp.RTC, &mut backup_domain);

    loop {
        let alr_time = rtc.current_time();
        let alr_time = alr_time + 10;
        println!("alr_time: {}", alr_time);
        rtc.set_alarm(alr_time);
        block!(rtc.wait_alarm()).unwrap();

        oled::show_string(&mut scl, &mut sda, 1, 1, "CNT:");
        oled::show_string(&mut scl, &mut sda, 2, 1, "ALR:");
        // 获取不到该状态
        // oled::show_string(&mut scl, &mut sda, 3, 1, "ALRF:");

        let count = rtc.current_time();
        println!("current_time: {}", count);
        oled::show_num(&mut scl, &mut sda, 1, 6, count, 10);
        oled::show_num(&mut scl, &mut sda, 2, 6, alr_time, 10);

        oled::show_string(&mut scl, &mut sda, 4, 1, "running");
        delay.delay_ms(100_u32);
        oled::show_string(&mut scl, &mut sda, 4, 1, "       ");
        delay.delay_ms(100_u32);

        oled::show_string(&mut scl, &mut sda, 4, 9, "STANDBY");
        delay.delay_ms(100_u32);
        oled::show_string(&mut scl, &mut sda, 4, 9, "       ");
        delay.delay_ms(100_u32);

        oled::clear(&mut scl, &mut sda);

        // 当CPU进入深度睡眠时进入待机模式
        pwr.cr
            .modify(|_, w| w.csbf().clear_bit().pdds().standby_mode());
    }
}

/// 初始化 OLED 显示屏
pub fn init_oled(
    pb8: gpio::Pin<'B', 8>,
    pb9: gpio::Pin<'B', 9>,
    crh: &mut gpio::Cr<'B', true>,
) -> (
    gpio::PB8<gpio::Output<gpio::OpenDrain>>,
    gpio::PB9<gpio::Output<gpio::OpenDrain>>,
) {
    // 将引脚配置为作为开漏输出模式
    let mut scl = pb8.into_open_drain_output(crh);
    let mut sda = pb9.into_open_drain_output(crh);
    scl.set_speed(crh, gpio::IOPinSpeed::Mhz50);
    sda.set_speed(crh, gpio::IOPinSpeed::Mhz50);

    // 始化 OLED 配置
    oled::init_oled_config(&mut scl, &mut sda);
    (scl, sda)
}