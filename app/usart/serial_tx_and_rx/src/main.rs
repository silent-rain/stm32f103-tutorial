#![no_std]
#![no_main]
#![allow(clippy::empty_loop)]

use cortex_m::prelude::_embedded_hal_blocking_delay_DelayMs;
use defmt::println;
use defmt_rtt as _;
use panic_probe as _;

use nb::block;

use cortex_m::asm::wfi;
use cortex_m_rt::entry;
use stm32f1xx_hal::pac;
use stm32f1xx_hal::prelude::_stm32_hal_afio_AfioExt;
use stm32f1xx_hal::prelude::_stm32_hal_flash_FlashExt;
use stm32f1xx_hal::prelude::_stm32_hal_gpio_GpioExt;
use stm32f1xx_hal::rcc::RccExt;
use stm32f1xx_hal::serial;
use stm32f1xx_hal::serial::Serial;
use stm32f1xx_hal::time::U32Ext;
use stm32f1xx_hal::timer::SysTimerExt;
use unwrap_infallible::UnwrapInfallible;

#[entry]
fn main() -> ! {
    // 获取对外设的访问对象
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();
    let mut afio = dp.AFIO.constrain();
    let syst = cp.SYST;

    let mut gpioa = dp.GPIOA.split();

    // 冻结系统中所有时钟的配置，并将冻结的频率存储在时钟中
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    // 具有自定义精度的阻塞延迟函数
    let mut delay = syst.delay(&clocks);

    // USART1
    let tx = gpioa.pa9.into_alternate_push_pull(&mut gpioa.crh);
    let rx = gpioa.pa10;

    // USART1
    // let tx = gpiob.pb6.into_alternate_push_pull(&mut gpiob.crl);
    // let rx = gpiob.pb7;

    // USART2
    // let tx = gpioa.pa2.into_alternate_push_pull(&mut gpioa.crl);
    // let rx = gpioa.pa3;

    // USART3
    // Configure pb10 as a push_pull output, this will be the tx pin
    // let tx = gpiob.pb10.into_alternate_push_pull(&mut gpiob.crh);
    // Take ownership over pb11
    // let rx = gpiob.pb11;

    // 设置usart设备。取得USART寄存器和tx/rx引脚的所有权。其余寄存器用于启用和配置设备。
    println!("load serial...");
    let mut serial = Serial::new(
        dp.USART1,
        (tx, rx),
        &mut afio.mapr,
        serial::Config::default().baudrate(9600.bps()),
        &clocks,
    );

    // 写入“X”，然后等待写入成功。
    let sent = b'X';
    block!(serial.tx.write(sent)).unwrap_infallible();
    println!("sent X");

    // 读取刚刚发送的字节。等待读取完成
    let received = block!(serial.rx.read()).unwrap();

    // 既然我们已经连接了tx和rx，那么我们发送的字节应该是我们接收的字节
    println!(
        "received = {:?}, sent = {:?}",
        received as char, sent as char
    );
    assert_eq!(received, sent);

    delay.delay_ms(1000_u32);

    let sent = b'Y';
    block!(serial.tx.write(sent)).unwrap_infallible();
    println!("sent Y");

    let received = block!(serial.rx.read()).unwrap();
    println!(
        "received = {:?}, sent = {:?}",
        received as char, sent as char
    );
    assert_eq!(received, sent);

    println!("loop");
    loop {
        wfi();
    }
}
