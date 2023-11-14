#![no_std]
#![no_main]
#![allow(clippy::empty_loop)]

use defmt::println;
use defmt_rtt as _;
use panic_probe as _;

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
    let mut _delay = syst.delay(&clocks);

    // USART1
    let tx = gpioa.pa9.into_alternate_push_pull(&mut gpioa.crh);
    let rx = gpioa.pa10;

    // 设置usart设备。取得USART寄存器和tx/rx引脚的所有权。其余寄存器用于启用和配置设备。
    println!("load serial...");
    let (mut tx, mut rx) = Serial::new(
        dp.USART1,
        (tx, rx),
        &mut afio.mapr,
        serial::Config::default().baudrate(9600.bps()),
        &clocks,
    )
    .split();

    // 发送
    println!("send");
    hardware::serial::send_byte(&mut tx, b'X');
    hardware::serial::send_byte(&mut tx, b'\n');
    hardware::serial::send_bytes(&mut tx, &[b'X', b'Y', b'Z', b'\n']);
    hardware::serial::send_bytes(&mut tx, "xyz\n".as_bytes());
    hardware::serial::send_string(&mut tx, "test\n");
    hardware::serial::send_number(&mut tx, 34567);
    println!("send end");
    // let number = 103;
    // writeln!(tx, "Hello formatted string {}", number).unwrap();

    println!("loop");
    loop {
        // let buffer: &mut [u8] = &mut [0; 5];
        // hardware::serial::recv_bytes(&mut rx, buffer);
        // println!("received = {:?}", buffer);
        // for c in buffer {
        //     println!("c = {:?}", *c as char);
        // }

        let received = hardware::serial::recv_string(&mut rx);
        let s = received.as_str();
        println!("received = {:#?}", s);
    }
}
