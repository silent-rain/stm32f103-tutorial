#![no_std]
#![no_main]
#![allow(clippy::empty_loop)]

use core::ptr::read_volatile;

use hardware::oled;

use defmt::println;
use defmt_rtt as _;
use panic_probe as _;

use cortex_m::asm::wfi;
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

    const FLASH_SIZE_REGISTER: *const u16 = 0x1FFFF7E0 as *const u16;
    const FLASH_UID_REGISTER: *const u16 = 0x1FFFF7E8 as *const u16;

    // 显示静态字符串
    oled.show_string(1, 1, "F_SIZE:");
    // 使用指针读取指定地址下的闪存容量寄存器
    let flash_size = unsafe { read_volatile(FLASH_SIZE_REGISTER) };
    println!("flash_size: {}", flash_size);
    oled.show_hex_num(1, 8, flash_size.into(), 4);

    // 显示静态字符串
    oled.show_string(2, 1, "U_ID:");
    // 使用指针读取指定地址下的产品唯一身份标识寄存器
    let flash_uid = unsafe { read_volatile(FLASH_UID_REGISTER) };
    oled.show_hex_num(2, 6, flash_uid.into(), 4);

    // 地址偏移显示
    oled.show_hex_num(
        2,
        11,
        unsafe { read_volatile((0x1FFFF7E8 + 0x02) as *const u16 as *const u32) },
        4,
    );
    oled.show_hex_num(
        3,
        1,
        unsafe { read_volatile((0x1FFFF7E8 + 0x04) as *const u32) },
        8,
    );
    oled.show_hex_num(
        4,
        1,
        unsafe { read_volatile((0x1FFFF7E8 + 0x08) as *const u32) },
        8,
    );

    loop {
        wfi();
    }
}
