#![no_std]
#![no_main]
#![allow(clippy::empty_loop)]

use core::convert::Infallible;

use defmt::println;
use defmt_rtt as _;
use panic_probe as _;

use cortex_m::prelude::_embedded_hal_blocking_delay_DelayMs;
use cortex_m_rt::entry;
use embedded_nrf24l01::{
    Configuration, CrcMode, DataRate, Payload, RxMode, StandbyMode, TxMode, NRF24L01,
};
use heapless::String;
use stm32f1xx_hal::{
    afio::MAPR,
    gpio::{self, Alternate, Input, Output, PullUp, PushPull, PB3, PB4, PB5, PB6, PB7},
    pac::{self, SPI1},
    prelude::{
        _fugit_RateExtU32, _stm32_hal_afio_AfioExt, _stm32_hal_flash_FlashExt,
        _stm32_hal_gpio_GpioExt,
    },
    rcc::{Clocks, RccExt},
    spi::{self, Spi, Spi1Remap},
    timer::SysTimerExt,
};

/// RF24L01 发送协议地址
pub const NRF24L01_TX_ADDR: &[u8] = b"fnord";
/// RF24L01 接收协议地址
pub const NRF24L01_RX_ADDR: &[u8] = b"fnord";
/// RF24L01 接收通道
pub const NRF24L01_RX_ADDR_P0: usize = 0x00;
pub const NRF24L01_RX_ADDR_P1: usize = 0x01;
pub const NRF24L01_RX_ADDR_P2: usize = 0x02;
pub const NRF24L01_RX_ADDR_P3: usize = 0x03;
pub const NRF24L01_RX_ADDR_P4: usize = 0x04;
pub const NRF24L01_RX_ADDR_P5: usize = 0x05;

type Device = NRF24L01<
    Infallible,
    PB6<Output<PushPull>>,
    PB7<Output<PushPull>>,
    Spi<SPI1, Spi1Remap, (PB3<Alternate>, PB4<Input<PullUp>>, PB5<Alternate>), u8>,
>;

pub type RxTY = RxMode<Device>;

/// NRF24L01 传输指令
#[derive(Debug)]
pub struct NRF24L01Cmd {
    pub cmd: i32,
}

/// 配置参数
pub struct Config<'a> {
    pub spi_sck: PB3,
    pub spi_miso: PB4,
    pub spi_mosi: PB5,
    pub nrf24_ce: PB6,
    pub nrf24_csn: PB7,
    pub crl: &'a mut gpio::Cr<'B', false>,
    pub spi1: SPI1,
    pub mapr: &'a mut MAPR,
    pub clocks: Clocks,
}

/// NRF24L01 2.4G 无线通信
pub struct Nrf24L01 {
    pub nrf24: StandbyMode<Device>,
}

impl Nrf24L01 {
    /// 初始化 NRF24L01 SPI 2.4 GHz 无线通信
    pub fn new(config: Config) -> Self {
        // 创建一个SPI实例
        let spi = {
            let sck = config.spi_sck.into_alternate_push_pull(config.crl);
            let miso = config.spi_miso.into_pull_up_input(config.crl);
            let mosi = config.spi_mosi.into_alternate_push_pull(config.crl);

            let mode = spi::Mode {
                polarity: spi::Polarity::IdleLow,
                phase: spi::Phase::CaptureOnFirstTransition,
            };

            Spi::spi1(
                config.spi1,
                (sck, miso, mosi),
                config.mapr,
                mode,
                1.MHz(),
                config.clocks,
            )
        };

        let ce = config.nrf24_ce.into_push_pull_output(config.crl);
        let csn = config.nrf24_csn.into_push_pull_output(config.crl);

        let nrf24 = NRF24L01::new(ce, csn, spi).unwrap();

        let mut nrf24l01 = Nrf24L01 { nrf24 };

        // 配置设备
        nrf24l01.init_config();

        nrf24l01
    }

    /// 配置设备
    fn init_config(&mut self) {
        // 设置频率为2.476 GHz
        // self.nrf24.set_frequency(76).unwrap();

        // 设置 nRF24 无线模块的通信速率为 2 Mbps，输出功率为 -18 dBm
        // RF output power in TX mode
        // * `00`: -18 dBm
        // * `01`: -12 dBm
        // * `10`: -6 dBm
        // * `11`: 0 dBm
        self.nrf24.set_rf(&DataRate::R250Kbps, 00).unwrap();
        // 关闭自动重传功能
        self.nrf24.set_auto_retransmit(0, 0).unwrap();
        // 设置CRC模式
        self.nrf24.set_crc(CrcMode::Disabled).unwrap();
        // 自动应答功能
        self.nrf24.set_auto_ack(&[true; 6]).unwrap();

        // 设置地址的长度，它可以是3，4或5字节
        // self.nrf24.set_pipes_rx_lengths(lengths);

        // 设置接收地址
        self.nrf24
            .set_rx_addr(NRF24L01_RX_ADDR_P0, NRF24L01_RX_ADDR)
            .unwrap();

        // 配置要启用或禁用接收管道
        // NRF24L01一共有6个管道，分别是0到5。
        // 每个管道都有一个5字节的地址，用来识别发送和接收的数据包。
        // 默认使用的是管道0
        self.nrf24.set_pipes_rx_enable(&[true; 6]).unwrap();

        // 设置发送地址
        self.nrf24.set_tx_addr(NRF24L01_TX_ADDR).unwrap();

        // 清空发送缓冲区
        // self.nrf24.flush_tx().unwrap();
        // 清空接收缓冲区
        // self.nrf24.flush_rx().unwrap();
    }

    /// 接收数据转换为字符串
    /// 最大长度: 32
    pub fn payload_string(payload: Payload) -> String<32> {
        let mut s: String<32> = String::new();
        let data = payload.as_ref();
        for byte in data {
            s.push(*byte as char).unwrap();
        }
        s
    }

    /// 接收数据包并返回有效载荷
    pub fn recv_data(rx: &mut RxMode<Device>) -> Option<Payload> {
        // 是否有数据包到达
        let _pipe = match rx.can_read() {
            Ok(v) => {
                println!("pipe {}", v);
                v
            }
            Err(_err) => return None,
        };
        // 接收数据包
        let payload = rx.read().unwrap();
        // let data: &[u8]  = payload.as_ref();
        // 处理接收到的数据包
        // println!("Received {} bytes on pipe {}", payload.len(), pipe);
        Some(payload)
    }

    /// 发送数据
    pub fn send_data(tx: &mut TxMode<Device>, bytes: &[u8]) {
        // 发送数据
        tx.send(bytes).expect("Failed to send data");

        // 等待队列清空
        while !tx.can_send().unwrap() {}

        // 清空发送缓冲区
        // tx.flush_tx().unwrap();

        // 等待队列清空
        // tx.wait_empty().unwrap();
    }
}

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

    let gpioa = dp.GPIOA.split();
    let mut gpiob = dp.GPIOB.split();

    // 冻结系统中所有时钟的配置，并将冻结的频率存储在时钟中
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    // 具有自定义精度的阻塞延迟函数
    let mut delay = syst.delay(&clocks);

    // 禁用 jtag 端口进行复用
    let (_pa15, pb3, pb4) = afio.mapr.disable_jtag(gpioa.pa15, gpiob.pb3, gpiob.pb4);

    // 初始化 NRF24L01 2.4 GHz 无线通信
    let nrf24l01 = Nrf24L01::new(Config {
        spi_sck: pb3,
        spi_miso: pb4,
        spi_mosi: gpiob.pb5,
        nrf24_ce: gpiob.pb6,
        nrf24_csn: gpiob.pb7,
        crl: &mut gpiob.crl,
        spi1,
        mapr: &mut afio.mapr,
        clocks,
    });
    // let mut rx = nrf24l01.nrf24.rx().unwrap();
    // println!("init nrf24l01_rx ...");

    let mut tx = nrf24l01.nrf24.tx().unwrap();
    println!("init nrf24l01 tx ...");

    loop {
        // let payload = Nrf24L01::recv_data(&mut rx).unwrap();
        // let data = Nrf24L01::payload_string(payload);
        // println!("{:?}", data.as_str());

        let bytes = "test".as_bytes();
        Nrf24L01::send_data(&mut tx, bytes);
        println!("send");
        delay.delay_ms(1000_u32);
    }
}
