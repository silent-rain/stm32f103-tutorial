#![no_std]
#![no_main]
#![allow(clippy::empty_loop)]

use hardware::oled;

use defmt::println;
use defmt_rtt as _;
use panic_probe as _;

use cortex_m_rt::entry;
use stm32f1xx_hal::flash;
use stm32f1xx_hal::gpio::gpiob;
use stm32f1xx_hal::pac;
use stm32f1xx_hal::prelude::_fugit_RateExtU32;
use stm32f1xx_hal::prelude::_stm32_hal_flash_FlashExt;
use stm32f1xx_hal::prelude::_stm32_hal_gpio_GpioExt;
use stm32f1xx_hal::rcc;
use stm32f1xx_hal::rcc::RccExt;

const CONST: i32 = 0x66;
static STATIC: i32 = 0x66;

#[entry]
fn main() -> ! {
    // 初始化外设
    // 获取对外设的访问对象
    let _cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    let mut flash: flash::Parts = dp.FLASH.constrain();
    let rcc: rcc::Rcc = dp.RCC.constrain();
    let adc1 = dp.ADC1;

    let mut gpiob: gpiob::Parts = dp.GPIOB.split();

    // 配置ADC时钟默认值是最慢的ADC时钟：PCLK2/8。同时ADC时钟可配置。
    // 因此，它的频率可能会被调整以满足某些实际需求。
    // 使用支持的预分频器值2/4/6/8来近似用户指定的值。
    let clocks = rcc.cfgr.adcclk(72.MHz()).freeze(&mut flash.acr);
    println!("adc freq: {:?}", clocks.adcclk().to_MHz());

    // 初始化 OLED 显示屏
    println!("load oled...");
    let mut oled = oled::simple::init_oled(gpiob.pb8, gpiob.pb9, &mut gpiob.crh);

    oled.show_string(1, 1, "v:");
    oled.show_string(2, 1, "c:");
    oled.show_string(3, 1, "s:");
    oled.show_string(4, 1, "a:");
    let var = 0x66;
    let var_p = &var as *const i32 as usize as u32;
    let const_p = &CONST as *const i32 as usize as u32;
    let static_p = &STATIC as *const i32 as usize as u32;
    let adc1_p = &adc1.dr as *const _ as usize as u32;
    println!(
        "var_p={:?} const_p={:?} static_p={:?} adc1_p={:?}",
        var_p, const_p, static_p, adc1_p
    );
    // SRAM
    // 0x20004B44
    oled.show_hex_num(1, 3, var_p, 8);
    // flush
    // 0x08007D60
    oled.show_hex_num(2, 3, const_p, 8);
    // flush
    // 0x08007D0C
    oled.show_hex_num(3, 3, static_p, 8);
    // 外设寄存器固定地址
    // 0x40012444C
    oled.show_hex_num(4, 3, adc1_p, 8);

    loop {}
}
