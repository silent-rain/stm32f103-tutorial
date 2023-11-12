#![no_std]
#![no_main]
#![allow(clippy::empty_loop)]

use core::cell::RefCell;

use defmt::println;
use defmt_rtt as _;
use hardware::oled;
use panic_probe as _;

use cortex_m::interrupt::Mutex;
use cortex_m::prelude::_embedded_hal_blocking_delay_DelayMs;
use cortex_m_rt::entry;
use stm32f1xx_hal::gpio;
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
use stm32f1xx_hal::timer::SysDelay;
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
    let mut gpiob = dp.GPIOB.split();

    // 冻结系统中所有时钟的配置，并将冻结的频率存储在时钟中
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    // 具有自定义精度的阻塞延迟函数
    let mut delay = syst.delay(&clocks);

    // 初始化 OLED 显示屏
    println!("load oled...");
    let mut oled = oled::simple::init_oled(gpiob.pb8, gpiob.pb9, &mut gpiob.crh);

    // 按键
    let mut key = gpiob.pb1.into_pull_up_input(&mut gpiob.crl);

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
        // 按键事件
        if get_key_status(&mut key, &mut delay) {
            println!("key");
            unsafe {
                SERIAL_TX_PACKET[0] += 1;
                SERIAL_TX_PACKET[1] += 1;
                SERIAL_TX_PACKET[2] += 1;
                SERIAL_TX_PACKET[3] += 1;
            }

            send_packet();

            oled.show_hex_num(2, 1, get_tx_packet(0), 2);
            oled.show_hex_num(2, 4, get_tx_packet(1), 2);
            oled.show_hex_num(2, 7, get_tx_packet(2), 2);
            oled.show_hex_num(2, 10, get_tx_packet(3), 2);
        }

        // 接收数据
        if get_rx_fkag() == RxFlag::End {
            oled.show_hex_num(4, 1, get_rx_packet(0), 2);
            oled.show_hex_num(4, 4, get_rx_packet(1), 2);
            oled.show_hex_num(4, 7, get_rx_packet(2), 2);
            oled.show_hex_num(4, 10, get_rx_packet(3), 2);
        }
    }
}

/// 获取按键的状态
/// 按键是否按下
fn get_key_status(
    key1: &mut gpio::Pin<'B', 1, gpio::Input<gpio::PullUp>>,
    delay: &mut SysDelay,
) -> bool {
    let mut key_num = false;

    if key1.is_low() {
        // 按键按下抖动
        delay.delay_ms(20_u16);
        // 按着不动, 松手后跳出循环
        while key1.is_low() {}
        // 按键松开抖动
        delay.delay_ms(20_u16);

        key_num = true;
    }
    key_num
}

/// 获取发送数据
fn get_tx_packet(index: usize) -> u32 {
    let data = unsafe { SERIAL_TX_PACKET[index] };
    data as u32
}

/// 接收起止标识符
#[derive(Debug, Clone, PartialEq)]
#[repr(u8)]
enum RxFlag {
    /// 开始标识: 0xFF
    Start,
    /// 结束标识:0xFE
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

// 发送数据包 FF090A0B0CFE
static mut SERIAL_TX_PACKET: [u8; 4] = [0; 4];
// 接收位置索引
static mut P_RX_PACKET: usize = 0;
// 接收数据包
// FF 01 02 03 04 FE
static mut SERIAL_RX_PACKET: [u8; 4] = [0; 4];
// 接收状态标识
static mut SERIAL_RX_FLAG: RxFlag = RxFlag::Start;
// 接收状态机
static mut SERIAL_RX_STATE: RxState = RxState::Wait;

/// 发送数据
fn send_packet() {
    cortex_m::interrupt::free(|cs| {
        if let Some(tx) = G_TX.borrow(cs).borrow_mut().as_mut() {
            hardware::serial::send_byte(tx, 0xFF);
            hardware::serial::send_bytes(tx, unsafe { &SERIAL_TX_PACKET });
            hardware::serial::send_byte(tx, 0xFE);
        }
    })
}

/// 获取发送标识
fn get_rx_fkag() -> RxFlag {
    unsafe { SERIAL_RX_FLAG.clone() }
}

/// 获取接收数据
fn get_rx_packet(index: usize) -> u32 {
    let data = unsafe { SERIAL_RX_PACKET[index] };
    data as u32
}

#[interrupt]
unsafe fn USART1() {
    cortex_m::interrupt::free(|cs| {
        if let Some(rx) = G_RX.borrow(cs).borrow_mut().as_mut() {
            if rx.is_rx_not_empty() {
                let rx_data = nb::block!(rx.read()).unwrap();
                match SERIAL_RX_STATE {
                    RxState::Wait => {
                        if rx_data == 0xFF {
                            SERIAL_RX_STATE = RxState::Receive;
                            P_RX_PACKET = 0;
                        }
                    }
                    RxState::Receive => {
                        SERIAL_RX_PACKET[P_RX_PACKET] = rx_data;
                        P_RX_PACKET += 1;
                        if P_RX_PACKET >= 4 {
                            SERIAL_RX_STATE = RxState::Finish;
                        }
                    }
                    RxState::Finish => {
                        if rx_data == 0xFE {
                            SERIAL_RX_STATE = RxState::Wait;
                            SERIAL_RX_FLAG = RxFlag::End;
                        }
                    }
                }
                rx.clear_idle_interrupt();
            }
        }
    })
}
