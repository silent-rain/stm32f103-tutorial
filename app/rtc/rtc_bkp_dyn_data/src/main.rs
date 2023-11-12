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
use stm32f1xx_hal::prelude::_stm32_hal_flash_FlashExt;
use stm32f1xx_hal::prelude::_stm32_hal_gpio_GpioExt;
use stm32f1xx_hal::rcc::RccExt;
use stm32f1xx_hal::rtc::Rtc;
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
    let mut pwr = dp.PWR;

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

    // 设置RTC
    // 启用对备份域的写入
    let mut backup_domain = rcc.bkp.constrain(dp.BKP, &mut pwr);

    // 从备份数据寄存器的DR1至DR10寄存器之一读取16位值。
    // 寄存器自变量是DRx寄存器的一个从零开始的索引：0表示DR1，最多9表示DR10。
    // 提供9以上的数字会引起恐慌
    let dr1 = backup_domain.read_data_register_low(0);
    if dr1 == 0 {
        // 将16位值写入备份数据寄存器的DR1至DR10寄存器之一。
        // 寄存器自变量是DRx寄存器的一个从零开始的索引：0表示DR1，最多9表示DR10。
        // 提供高于9的数字会引起恐慌。
        backup_domain.write_data_register_low(0, 10);
        backup_domain.write_data_register_low(1, 20);
    }

    // 启动RTC
    let mut rtc = Rtc::new(dp.RTC, &mut backup_domain);

    // 将当前时间设置为0
    rtc.set_time(0);

    oled.show_string(1, 1, "R:");
    oled.show_string(2, 1, "W:");

    let mut array_write = [0x1234, 0x5678];
    loop {
        // 按键事件
        if get_key_status(&mut key, &mut delay) {
            array_write[0] += 1;
            array_write[1] += 1;

            backup_domain.write_data_register_low(0, array_write[0]);
            backup_domain.write_data_register_low(1, array_write[1]);

            oled.show_num(1, 3, array_write[0] as u32, 4);
            oled.show_num(1, 8, array_write[1] as u32, 4);
        }

        let dr1 = backup_domain.read_data_register_low(0);
        let dr2 = backup_domain.read_data_register_low(1);

        oled.show_num(2, 3, dr1 as u32, 4);
        oled.show_num(2, 8, dr2 as u32, 4);
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
