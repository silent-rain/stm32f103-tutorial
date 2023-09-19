#![no_std]
#![no_main]
#![allow(clippy::empty_loop)]

use core::cell::RefCell;

use defmt::println;
use defmt_rtt as _;
use panic_probe as _;

use cortex_m::asm::wfi;
use cortex_m::interrupt::Mutex;
use cortex_m_rt::entry;
use stm32f1xx_hal::pac;
use stm32f1xx_hal::pac::interrupt;
use stm32f1xx_hal::pac::USART1;
use stm32f1xx_hal::prelude::_stm32_hal_afio_AfioExt;
use stm32f1xx_hal::prelude::_stm32_hal_flash_FlashExt;
use stm32f1xx_hal::prelude::_stm32_hal_gpio_GpioExt;
use stm32f1xx_hal::rcc::RccExt;
use stm32f1xx_hal::serial;
use stm32f1xx_hal::serial::Rx;
use stm32f1xx_hal::serial::Serial;
use stm32f1xx_hal::serial::Tx;
use stm32f1xx_hal::time::U32Ext;
use stm32f1xx_hal::timer::SysTimerExt;

static G_RX: Mutex<RefCell<Option<Rx<USART1>>>> = Mutex::new(RefCell::new(None));
static G_TX: Mutex<RefCell<Option<Tx<USART1>>>> = Mutex::new(RefCell::new(None));

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
        serial::Config::default()
            .baudrate(9600.bps())
            .stopbits(serial::StopBits::STOP2)
            .wordlength_9bits()
            .parity_odd(),
        &clocks,
    )
    .split();

    tx.listen();
    rx.listen();
    rx.listen_idle();

    cortex_m::interrupt::free(|cs| {
        G_TX.borrow(cs).replace(Some(tx));
        G_RX.borrow(cs).replace(Some(rx));
    });

    unsafe {
        cortex_m::peripheral::NVIC::unmask(pac::Interrupt::USART1);
    }

    loop {
        wfi();
    }
}

const BUFFER_LEN: usize = 4096;
static mut BUFFER: &mut [u8; BUFFER_LEN] = &mut [0; BUFFER_LEN];
static mut WIDX: usize = 0;

fn write(buf: &[u8]) {
    cortex_m::interrupt::free(|cs| {
        if let Some(tx) = G_TX.borrow(cs).borrow_mut().as_mut() {
            buf.iter()
                .for_each(|w| if let Err(_err) = nb::block!(tx.write(*w)) {})
        }
    })
}

#[interrupt]
unsafe fn USART1() {
    cortex_m::interrupt::free(|cs| {
        if let Some(rx) = G_RX.borrow(cs).borrow_mut().as_mut() {
            if rx.is_rx_not_empty() {
                if let Ok(w) = nb::block!(rx.read()) {
                    BUFFER[WIDX] = w;
                    WIDX += 1;
                    if WIDX >= BUFFER_LEN - 1 {
                        write(&BUFFER[..]);
                        WIDX = 0;
                    }
                }
                rx.listen_idle();
            } else if rx.is_idle() {
                // 如果设置了线路空闲状态，则返回true
                rx.unlisten_idle();
                write(&BUFFER[0..WIDX]);
                WIDX = 0;
            }
        }
    })
}
