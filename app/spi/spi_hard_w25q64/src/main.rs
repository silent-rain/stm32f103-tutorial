#![no_std]
#![no_main]
#![allow(clippy::empty_loop)]

use hardware::{oled, w25q64::w25q64_hal};

use defmt::println;
use defmt_rtt as _;
use panic_probe as _;

use cortex_m::asm::wfi;
use cortex_m_rt::entry;
use stm32f1xx_hal::gpio::{IOPinSpeed, OutputSpeed};
use stm32f1xx_hal::pac;
use stm32f1xx_hal::prelude::_embedded_hal_blocking_delay_DelayMs;
use stm32f1xx_hal::prelude::{
    _stm32_hal_afio_AfioExt, _stm32_hal_flash_FlashExt, _stm32_hal_gpio_GpioExt,
};
use stm32f1xx_hal::rcc::RccExt;
use stm32f1xx_hal::timer::SysTimerExt;

#[entry]
fn main() -> ! {
    // 获取对外设的访问对象
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();
    let mut afio = dp.AFIO.constrain();
    let syst = cp.SYST;
    let spi1 = dp.SPI1;

    let mut gpioa = dp.GPIOA.split();
    let mut gpiob = dp.GPIOB.split();

    // 冻结系统中所有时钟的配置，并将冻结的频率存储在时钟中
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    // 具有自定义精度的阻塞延迟函数
    let mut delay = syst.delay(&clocks);

    // 初始化 OLED 显示屏
    println!("load oled...");
    let mut oled = oled::simple::init_oled(gpiob.pb8, gpiob.pb9, &mut gpiob.crh);

    // 将PA4引脚初始化为推挽输出
    let mut cs = gpioa.pa4.into_push_pull_output(&mut gpioa.crl);
    cs.set_speed(&mut gpioa.crl, IOPinSpeed::Mhz50);

    // 将PA5引脚初始化为复用推挽输出
    let mut sck = gpioa.pa5.into_alternate_push_pull(&mut gpioa.crl);
    sck.set_speed(&mut gpioa.crl, IOPinSpeed::Mhz50);
    // 将PA6引脚初始化为上拉输入
    let miso = gpioa.pa6.into_pull_up_input(&mut gpioa.crl);
    // 不使用 MISO 引脚
    // let miso = NoMiso;
    // 将PA7引脚初始化为复用推挽输出
    let mut mosi = gpioa.pa7.into_alternate_push_pull(&mut gpioa.crl);
    mosi.set_speed(&mut gpioa.crl, IOPinSpeed::Mhz50);

    // 创建一个Spi实例
    let pins = (sck, miso, mosi);
    let mut w25q = w25q64_hal::W25Q64::new(spi1, pins, &mut cs, &mut afio.mapr, clocks);

    delay.delay_ms(1000_u32);

    // 读取芯片的JEDEC设备ID
    let (manufacturer_id, memory_type, capacity) = w25q.read_jedec_device_id().unwrap();
    println!(
        "manufacturer_id: {:02X}, memory_type: {:02X}, capacity: {:02X}",
        manufacturer_id, memory_type, capacity
    );

    // 读取芯片的制造商和设备ID
    let (manufacturer_id, device_id) = w25q.read_manufacturer_device_id().unwrap();
    println!(
        "manufacturer_id: {:02X}, device_id: {:02X}",
        manufacturer_id, device_id
    );

    // 检查是否有写保护标志
    // let protect = w25q.check_write_protect().unwrap();
    // println!("protect: {:?}", protect);

    // 擦除地址所在的扇区
    println!("sector_erase ...");
    w25q.sector_erase(0x000000).unwrap();
    // w25q.erase_chip().unwrap();

    delay.delay_ms(1000_u32);

    // 写入数据
    println!("page_program ...");
    let array_write = [0x01, 0x02, 0x03, 0x04];
    w25q.page_program(0x000000, &array_write).unwrap();

    // 读取数据
    let mut buffer = [0; 4];
    w25q.read_data(0x000000, &mut buffer).unwrap();
    println!("read_data: {:?}", buffer);

    oled.show_string(1, 1, "MID:   DID:");
    oled.show_string(2, 1, "TYP:   CAP:");
    oled.show_string(3, 1, "W:");
    oled.show_string(4, 1, "R:");

    oled.show_hex_num(1, 5, manufacturer_id as u32, 2);
    oled.show_hex_num(1, 12, device_id as u32, 4);

    oled.show_hex_num(2, 5, memory_type as u32, 2);
    oled.show_hex_num(2, 12, capacity as u32, 4);

    // 显示写入数据的测试数组
    oled.show_hex_num(3, 3, array_write[0] as u32, 2);
    oled.show_hex_num(3, 6, array_write[1] as u32, 2);
    oled.show_hex_num(3, 9, array_write[2] as u32, 2);
    oled.show_hex_num(3, 12, array_write[3] as u32, 2);

    // 显示读取数据的测试数组
    oled.show_hex_num(4, 3, buffer[0] as u32, 2);
    oled.show_hex_num(4, 6, buffer[1] as u32, 2);
    oled.show_hex_num(4, 9, buffer[2] as u32, 2);
    oled.show_hex_num(4, 12, buffer[3] as u32, 2);

    loop {
        wfi();
    }
}
