#![allow(clippy::empty_loop)]
#![no_std]
#![no_main]

use hardware::{oled, w25q64::w25q64_hal};

use defmt::println;
use defmt_rtt as _;
use panic_probe as _;

use cortex_m::asm::wfi;
use cortex_m_rt::entry;
use stm32f1xx_hal::gpio;
use stm32f1xx_hal::gpio::OutputSpeed;
use stm32f1xx_hal::pac;
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
    let mut _delay = syst.delay(&clocks);

    // 初始化 OLED 显示屏
    println!("load oled...");
    let (mut scl, mut sda) = init_oled(gpiob.pb8, gpiob.pb9, &mut gpiob.crh);

    // 配置SPI1的引脚和模式
    let sck = gpioa.pa5.into_alternate_push_pull(&mut gpioa.crl);
    let miso = gpioa.pa6.into_floating_input(&mut gpioa.crl);
    let mosi = gpioa.pa7.into_alternate_push_pull(&mut gpioa.crl);
    let mut cs = gpioa.pa4.into_push_pull_output(&mut gpioa.crl); // 配置片选引脚为推挽输出
    let pins = (sck, miso, mosi);

    // 创建一个Spi实例
    let mut spi = w25q64_hal::init_w25q64(spi1, pins, &mut afio.mapr, clocks);

    // 读取W25Q64芯片的MID和DID
    let (mid, did) = w25q64_hal::read_device_id(&mut spi, &mut cs).unwrap();

    // 启用写入功能
    w25q64_hal::write_enable(&mut spi, &mut cs).unwrap();

    // 擦除地址所在的扇区
    let address = 0x0000_0000;
    w25q64_hal::sector_erase(&mut spi, &mut cs, address).unwrap();

    // 写入数据
    let address = 0x0000_0000;
    let array_write = [0x01, 0x02, 0x03, 0x04];
    w25q64_hal::page_program(&mut spi, &mut cs, address, &array_write).unwrap();

    // 禁用写入功能
    w25q64_hal::write_disable(&mut spi, &mut cs).unwrap();

    // 读取数据
    let address = 0x0000_0000;
    let mut buffer = [0x00; 4];
    w25q64_hal::read_data(&mut spi, &mut cs, address, &mut buffer).unwrap();

    oled::show_string(&mut scl, &mut sda, 1, 1, "MID:   DID:");
    oled::show_string(&mut scl, &mut sda, 2, 1, "W:");
    oled::show_string(&mut scl, &mut sda, 3, 1, "R:");

    oled::show_hex_num(&mut scl, &mut sda, 1, 5, mid as u32, 2);
    oled::show_hex_num(&mut scl, &mut sda, 1, 12, did as u32, 4);

    oled::show_hex_num(&mut scl, &mut sda, 2, 3, array_write[0] as u32, 2);
    oled::show_hex_num(&mut scl, &mut sda, 2, 6, array_write[1] as u32, 2);
    oled::show_hex_num(&mut scl, &mut sda, 2, 9, array_write[2] as u32, 2);
    oled::show_hex_num(&mut scl, &mut sda, 2, 12, array_write[3] as u32, 2);

    oled::show_hex_num(&mut scl, &mut sda, 3, 3, buffer[0] as u32, 2);
    oled::show_hex_num(&mut scl, &mut sda, 3, 6, buffer[1] as u32, 2);
    oled::show_hex_num(&mut scl, &mut sda, 3, 9, buffer[2] as u32, 2);
    oled::show_hex_num(&mut scl, &mut sda, 3, 12, buffer[3] as u32, 2);

    loop {
        wfi();
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
