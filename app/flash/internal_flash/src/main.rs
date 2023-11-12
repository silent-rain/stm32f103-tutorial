#![no_std]
#![no_main]
#![allow(clippy::empty_loop)]

use hardware::{flash_store::FlashStore, key::get_key_status, oled};

use defmt::println;
use defmt_rtt as _;
use panic_probe as _;

use cortex_m_rt::entry;
use stm32f1xx_hal::{
    pac,
    prelude::{
        _embedded_hal_blocking_delay_DelayMs, _stm32_hal_flash_FlashExt, _stm32_hal_gpio_GpioExt,
    },
    rcc::RccExt,
    timer::SysTimerExt,
};

#[entry]
fn main() -> ! {
    // 获取对外设的访问对象
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();
    let syst = cp.SYST;

    let mut gpiob = dp.GPIOB.split();

    // 冻结系统中所有时钟的配置，并将冻结的频率存储在时钟中
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    // 具有自定义精度的阻塞延迟函数
    let mut delay = syst.delay(&clocks);

    // 初始化 OLED 显示屏
    println!("load oled...");
    let mut oled = oled::simple::init_oled(gpiob.pb8, gpiob.pb9, &mut gpiob.crh);

    // 调试延迟
    delay.delay_ms(1000_u32);

    // 按键
    let mut key1 = gpiob.pb1.into_pull_up_input(&mut gpiob.crl);
    let mut key2 = gpiob.pb11.into_pull_up_input(&mut gpiob.crh);

    let flash_store = FlashStore::new();
    // 参数存储模块初始化，在上电的时候将闪存的数据加载回Store_Data，实现掉电不丢失
    flash_store.init_store();

    oled.show_string(1, 1, "Flag:");
    oled.show_string(2, 1, "Data:");

    loop {
        // 获取按键状态
        // 按键1按下
        if get_key_status(&mut key1, &mut delay) {
            // 变换测试数据
            flash_store.set_store(1, flash_store.get_store(1) + 1);
            flash_store.set_store(2, flash_store.get_store(2) + 2);
            flash_store.set_store(3, flash_store.get_store(3) + 3);
            flash_store.set_store(4, flash_store.get_store(4) + 4);

            // 将Store_Data的数据备份保存到闪存，实现掉电不丢失
            flash_store.store_save();
        }

        // 按键2按下
        if get_key_status(&mut key2, &mut delay) {
            // 将Store_Data的数据全部清0
            flash_store.store_clear();
        }

        // 显示Store_Data的第一位标志位
        oled.show_hex_num(1, 6, flash_store.get_store(0).into(), 4);
        // 显示Store_Data的有效存储数据
        oled.show_hex_num(3, 1, flash_store.get_store(1).into(), 4);
        oled.show_hex_num(3, 6, flash_store.get_store(2).into(), 4);
        oled.show_hex_num(4, 1, flash_store.get_store(3).into(), 4);
        oled.show_hex_num(4, 6, flash_store.get_store(4).into(), 4);
    }
}
