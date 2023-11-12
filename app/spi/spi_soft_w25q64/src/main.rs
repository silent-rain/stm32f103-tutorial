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
    let mut oled = oled::simple::init_oled(gpiob.pb8, gpiob.pb9, &mut gpiob.crh);

    // 初始化 W25Q64
    // 推挽输出模式
    let mut w_ss = gpioa.pa4.into_push_pull_output(&mut gpioa.crl);
    // 推挽输出模式
    let mut w_sck = gpioa.pa5.into_push_pull_output(&mut gpioa.crl);
    // 推挽输出模式
    let mut w_mosi = gpioa.pa7.into_push_pull_output(&mut gpioa.crl);
    // 上拉输入模式
    let mut w_miso = gpioa.pa6.into_pull_up_input(&mut gpioa.crl);
    w_ss.set_speed(&mut gpioa.crl, gpio::IOPinSpeed::Mhz50);
    w_sck.set_speed(&mut gpioa.crl, gpio::IOPinSpeed::Mhz50);
    w_mosi.set_speed(&mut gpioa.crl, gpio::IOPinSpeed::Mhz50);
    let mut w25 = w25q64_reg::W25Q64::new(&mut w_ss, &mut w_sck, &mut w_mosi, &mut w_miso);

    // let (mid, did) = w25.read_id();
    // println!("mid: {:02X}, did: {:02X}", mid, did);

    // 读取芯片的JEDEC设备ID
    let (manufacturer_id, memory_type, capacity) = w25.read_jedec_device_id();
    println!(
        "manufacturer_id: {:02X}, memory_type: {:02X}, capacity: {:02X}",
        manufacturer_id, memory_type, capacity
    );

    // 读取芯片的制造商和设备ID
    let (manufacturer_id, device_id) = w25.read_manufacturer_device_id();
    println!(
        "manufacturer_id: {:02X}, device_id: {:02X}",
        manufacturer_id, device_id
    );

    let array_write: [u8; 4] = [0x01, 0x02, 0x03, 0x04];
    let mut array_read: [u8; 4] = [0; 4];

    println!("sector_erase");
    w25.sector_erase(0x000000);
    println!("page_program");
    w25.page_program(0x000000, &array_write);

    w25.read_data(0x000000, &mut array_read);

    oled.show_string(1, 1, "MID:   DID:");
    oled.show_string(2, 1, "TYP:   CAP:");
    oled.show_string(3, 1, "W:");
    oled.show_string(4, 1, "R:");

    oled.show_hex_num(1, 5, manufacturer_id as u32, 2);
    oled.show_hex_num(1, 12, device_id as u32, 4);
    oled.show_hex_num(2, 5, memory_type as u32, 2);
    oled.show_hex_num(2, 12, capacity as u32, 4);

    oled.show_hex_num(3, 3, array_write[0] as u32, 2);
    oled.show_hex_num(3, 6, array_write[1] as u32, 2);
    oled.show_hex_num(3, 9, array_write[2] as u32, 2);
    oled.show_hex_num(3, 12, array_write[3] as u32, 2);

    oled.show_hex_num(4, 3, array_read[0] as u32, 2);
    oled.show_hex_num(4, 6, array_read[1] as u32, 2);
    oled.show_hex_num(4, 9, array_read[2] as u32, 2);
    oled.show_hex_num(4, 12, array_read[3] as u32, 2);

    loop {
        wfi();
    }
}
