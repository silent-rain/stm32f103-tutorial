#![no_std]
#![no_main]
#![allow(clippy::empty_loop)]

use core::cell::RefCell;

use hardware::oled;

use defmt::println;
use defmt_rtt as _;
use panic_probe as _;

use cortex_m::interrupt::Mutex;
use cortex_m_rt::entry;
use stm32f1xx_hal::gpio;
use stm32f1xx_hal::gpio::OutputSpeed;
use stm32f1xx_hal::pac;
use stm32f1xx_hal::pac::interrupt;
use stm32f1xx_hal::pac::USART1;
use stm32f1xx_hal::prelude::{
    _stm32_hal_afio_AfioExt, _stm32_hal_flash_FlashExt, _stm32_hal_gpio_GpioExt,
};
use stm32f1xx_hal::rcc::RccExt;
use stm32f1xx_hal::serial;
use stm32f1xx_hal::serial::Rx;
use stm32f1xx_hal::serial::Serial;
use stm32f1xx_hal::serial::Tx;
use stm32f1xx_hal::time::U32Ext;
use stm32f1xx_hal::timer::SysTimerExt;

use heapless::String;

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
    let mut gpiob = dp.GPIOB.split();

    // 冻结系统中所有时钟的配置，并将冻结的频率存储在时钟中
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    // 具有自定义精度的阻塞延迟函数
    let mut _delay = syst.delay(&clocks);

    // 初始化 OLED 显示屏
    println!("load oled...");
    let mut oled = oled::simple::init_oled(gpiob.pb8, gpiob.pb9, &mut gpiob.crh);

    let mut led = gpioa.pa0.into_push_pull_output(&mut gpioa.crl);
    // 设置其输出速度（50 MHz）。
    led.set_speed(&mut gpioa.crl, gpio::IOPinSpeed::Mhz50);
    // 默认关闭LED
    led.set_high();

    // USART1
    let tx = gpioa.pa9.into_alternate_push_pull(&mut gpioa.crh);
    let rx = gpioa.pa10;

    // 设置usart设备。取得USART寄存器和tx/rx引脚的所有权。其余寄存器用于启用和配置设备。
    println!("load serial...");
    let (tx, mut rx) = Serial::new(
        dp.USART1,
        (tx, rx),
        &mut afio.mapr,
        serial::Config::default()
            .baudrate(9600.bps())
            .stopbits(serial::StopBits::STOP2)
            .wordlength_9bits()
            .parity_none(),
        &clocks,
    )
    .split();

    rx.listen();

    cortex_m::interrupt::free(|cs| {
        G_TX.borrow(cs).replace(Some(tx));
        G_RX.borrow(cs).replace(Some(rx));
    });

    unsafe {
        cortex_m::peripheral::NVIC::unmask(pac::Interrupt::USART1);
    }

    oled.show_string(1, 1, "TxPacket");
    oled.show_string(3, 1, "RxPacket");
    loop {
        // 接收数据
        if get_rx_fkag() == RxFlag::End {
            oled.show_string(4, 1, "                ");
            oled.show_string(4, 1, "                ");

            if get_rx_packet().as_str().trim_end_matches('\0') == "LED_ON" {
                led.set_low();
                send_packet("LED_ON_OK\r\n");
                oled.show_string(2, 1, "                ");
                oled.show_string(2, 1, "LED_ON_OK");
            } else if get_rx_packet().as_str().trim_end_matches('\0') == "LED_OFF" {
                led.set_high();
                send_packet("LED_OFF_OK\r\n");
                oled.show_string(2, 1, "                ");
                oled.show_string(2, 1, "LED_OFF_OK");
            } else {
                send_packet("ERROR_COMMAND\r\n");
                oled.show_string(2, 1, "                ");
                oled.show_string(2, 1, "ERROR_COMMAND");
            }

            unsafe {
                SERIAL_RX_FLAG = RxFlag::Start;
            }
        }
    }
}

/// 发送数据
fn send_packet(words: &str) {
    cortex_m::interrupt::free(|cs| {
        if let Some(tx) = G_TX.borrow(cs).borrow_mut().as_mut() {
            hardware::serial::send_string(tx, words);
        }
    })
}

/// 接收起止标识符
#[derive(Debug, Clone, PartialEq)]
#[repr(u8)]
enum RxFlag {
    /// 开始标识: @
    Start,
    /// 结束标识: \n
    End,
}

/// 接收状态机
#[derive(Debug, Clone, PartialEq)]
#[repr(u8)]
enum RxState {
    /// 等待接收
    Wait,
    /// 接收中
    Receive,
    /// 完成接收
    Finish,
}

// 接收位置索引
static mut P_RX_PACKET: usize = 0;
// 接收数据包
// FF 01 02 03 04 FE
static mut SERIAL_RX_PACKET: [u8; 1024] = [0; 1024];
// 接收状态标识
static mut SERIAL_RX_FLAG: RxFlag = RxFlag::Start;
// 接收状态机
static mut SERIAL_RX_STATE: RxState = RxState::Wait;

/// 获取发送标识
fn get_rx_fkag() -> RxFlag {
    unsafe { SERIAL_RX_FLAG.clone() }
}

/// 获取接收数据
fn get_rx_packet() -> String<1024> {
    let data = unsafe { SERIAL_RX_PACKET };
    let mut s: String<1024> = String::new();
    for c in data {
        if c == b'\n' {
            break;
        }
        s.push(c as char).unwrap();
    }
    s
}

#[interrupt]
unsafe fn USART1() {
    cortex_m::interrupt::free(|cs| {
        if let Some(rx) = G_RX.borrow(cs).borrow_mut().as_mut() {
            if rx.is_rx_not_empty() {
                let rx_data = nb::block!(rx.read()).unwrap();
                match SERIAL_RX_STATE {
                    RxState::Wait => {
                        if rx_data == b'@' && SERIAL_RX_FLAG == RxFlag::Start {
                            SERIAL_RX_STATE = RxState::Receive;
                            P_RX_PACKET = 0;
                        }
                    }
                    RxState::Receive => {
                        if rx_data == b'\r' {
                            SERIAL_RX_STATE = RxState::Finish;
                        } else {
                            SERIAL_RX_PACKET[P_RX_PACKET] = rx_data;
                            P_RX_PACKET += 1;
                        }
                    }
                    RxState::Finish => {
                        if rx_data == b'\n' {
                            SERIAL_RX_STATE = RxState::Wait;
                            SERIAL_RX_PACKET[P_RX_PACKET] = b'\0';
                            SERIAL_RX_FLAG = RxFlag::End;
                        }
                    }
                }
                rx.clear_idle_interrupt();
            }
        }
    })
}
