#![no_std]
#![no_main]
#![allow(clippy::empty_loop)]

use hardware::oled;

use defmt::println;
use defmt_rtt as _;
use panic_probe as _;

use cortex_m::asm::wfi;
use cortex_m::prelude::_embedded_hal_blocking_delay_DelayMs;
use cortex_m_rt::entry;
use nb::block;
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
    let mut gpiob = dp.GPIOB.split();

    // 冻结系统中所有时钟的配置，并将冻结的频率存储在时钟中
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    // 具有自定义精度的阻塞延迟函数
    let mut delay = syst.delay(&clocks);

    // 初始化 OLED 显示屏
    let mut oled = oled::simple::init_oled(gpiob.pb8, gpiob.pb9, &mut gpiob.crh);

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

    oled.show_string(1, 1, "RxData:");
    println!("loop");
    loop {
        if rx.is_rx_not_empty() {
            let w = block!(rx.read()).unwrap();
            hardware::serial::send_byte(&mut tx, w);
            println!("received = {:#?}", w);
            oled.show_hex_num(1, 8, w as u32, 2);
        }

        oled.show_string(2, 1, "running");
        delay.delay_ms(100_u32);
        oled.show_string(2, 1, "       ");
        delay.delay_ms(100_u32);

        // wfe(); // 事件唤醒
        wfi(); // 中断唤醒（推荐）
    }
}
