#![no_std]
#![no_main]
#![allow(clippy::empty_loop)]

use hardware::{oled, w25q64::w25q64_reg};

use defmt::println;
use defmt_rtt as _;
use panic_probe as _;

use cortex_m::asm::wfi;
use cortex_m_rt::entry;
use stm32f1xx_hal::gpio;
use stm32f1xx_hal::gpio::OutputSpeed;
use stm32f1xx_hal::pac;
use stm32f1xx_hal::prelude::{_stm32_hal_flash_FlashExt, _stm32_hal_gpio_GpioExt};
use stm32f1xx_hal::rcc::RccExt;
use stm32f1xx_hal::timer::SysTimerExt;

#[entry]
fn main() -> ! {
    // 获取对外设的访问对象
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();
    let syst = cp.SYST;

    let mut gpioa = dp.GPIOA.split();
    let mut gpiob = dp.GPIOB.split();

    // 冻结系统中所有时钟的配置，并将冻结的频率存储在时钟中
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    // 具有自定义精度的阻塞延迟函数
    let mut _delay = syst.delay(&clocks);

    // 初始化 OLED 显示屏
    println!("load oled...");
    let (mut scl, mut sda) = oled::simple::init_oled_pin(gpiob.pb8, gpiob.pb9, &mut gpiob.crh);
    let mut oled = oled::OLED::new(&mut scl, &mut sda);

    // 初始化 W25Q64
    let mut w_ss = gpioa.pa4.into_push_pull_output(&mut gpioa.crl);
    let mut w_sck = gpioa.pa5.into_push_pull_output(&mut gpioa.crl);
    let mut w_mosi = gpioa.pa7.into_push_pull_output(&mut gpioa.crl);
    let mut w_miso = gpioa.pa6.into_pull_up_input(&mut gpioa.crl);
    w_ss.set_speed(&mut gpioa.crl, gpio::IOPinSpeed::Mhz50);
    w_sck.set_speed(&mut gpioa.crl, gpio::IOPinSpeed::Mhz50);
    w_mosi.set_speed(&mut gpioa.crl, gpio::IOPinSpeed::Mhz50);
    let mut w25 = w25q64_reg::W25Q64::new(&mut w_ss, &mut w_sck, &mut w_mosi, &mut w_miso);

    let (mid, did) = w25.read_id();
    println!("mid: {:?}, did: {:?}", mid, did);

    let array_write: [u8; 4] = [0x01, 0x02, 0x03, 0x04];
    let mut array_read: [u8; 4] = [0; 4];

    w25.sector_erase(0x000000);
    w25.page_program(0x000000, &array_write);

    w25.read_data(0x000000, &mut array_read);

    oled.show_string(1, 1, "MID:   DID:");
    oled.show_string(2, 1, "W:");
    oled.show_string(3, 1, "R:");

    oled.show_hex_num(1, 5, mid as u32, 2);
    oled.show_hex_num(1, 12, did as u32, 4);

    oled.show_hex_num(2, 3, array_write[0] as u32, 2);
    oled.show_hex_num(2, 6, array_write[1] as u32, 2);
    oled.show_hex_num(2, 9, array_write[2] as u32, 2);
    oled.show_hex_num(2, 12, array_write[3] as u32, 2);

    oled.show_hex_num(3, 3, array_read[0] as u32, 2);
    oled.show_hex_num(3, 6, array_read[1] as u32, 2);
    oled.show_hex_num(3, 9, array_read[2] as u32, 2);
    oled.show_hex_num(3, 12, array_read[3] as u32, 2);

    loop {
        wfi();
    }
}
