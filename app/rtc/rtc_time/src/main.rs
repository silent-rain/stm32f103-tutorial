#![allow(clippy::empty_loop)]
#![no_std]
#![no_main]

use hardware::oled;

use defmt::println;
use defmt_rtt as _;
use panic_probe as _;

use cortex_m_rt::entry;
use stm32f1xx_hal::gpio;
use stm32f1xx_hal::gpio::OutputSpeed;
use stm32f1xx_hal::pac;
use stm32f1xx_hal::prelude::_embedded_hal_blocking_delay_DelayMs;
use stm32f1xx_hal::prelude::_stm32_hal_flash_FlashExt;
use stm32f1xx_hal::prelude::_stm32_hal_gpio_GpioExt;
use stm32f1xx_hal::rcc::RccExt;
use stm32f1xx_hal::rtc::Rtc;
use stm32f1xx_hal::timer::SysTimerExt;
use time::macros::datetime;
use time::OffsetDateTime;

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

    let mut backup_domain = rcc.bkp.constrain(dp.BKP, &mut pwr);
    let mut rtc = Rtc::new(dp.RTC, &mut backup_domain);

    // 设置初始时间
    if rtc.current_time() == 0 {
        let dt = datetime!(2023-10-20 12:30:00 UTC);
        let timestamp = dt.unix_timestamp();
        rtc.set_time(timestamp as u32);
    }

    oled::show_string(&mut scl, &mut sda, 1, 1, "Date:XXXX-XX-XX");
    oled::show_string(&mut scl, &mut sda, 2, 1, "Time:XX:XX:XX");
    oled::show_string(&mut scl, &mut sda, 3, 1, "CNT :");
    loop {
        let timestamp = rtc.current_time() as i64;
        println!("timestamp: {}", timestamp);
        // 创建 OffsetDateTime 对象，假定时间戳是以秒为单位, UTC
        // let timestamp = timestamp + 8 * 60 * 60;
        let datetime = OffsetDateTime::from_unix_timestamp(timestamp).unwrap();

        // 格式化日期时间为字符串
        let year = datetime.year() as u32;
        let month = datetime.month() as u32;
        let day = datetime.day() as u32;
        let hour = datetime.hour() as u32;
        let minute = datetime.minute() as u32;
        let second = datetime.second() as u32;
        println!(
            "time: {:?}-{:?}-{:?} {:?}:{:?}:{:?}",
            year, month, day, hour, minute, second
        );

        oled::show_num(&mut scl, &mut sda, 1, 6, year, 4);
        oled::show_num(&mut scl, &mut sda, 1, 11, month, 2);
        oled::show_num(&mut scl, &mut sda, 1, 14, day, 2);
        oled::show_num(&mut scl, &mut sda, 2, 6, hour, 2);
        oled::show_num(&mut scl, &mut sda, 2, 9, minute, 2);
        oled::show_num(&mut scl, &mut sda, 2, 12, second, 2);

        oled::show_num(&mut scl, &mut sda, 3, 6, timestamp as u32, 10);
        delay.delay_ms(1000_u32);
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
