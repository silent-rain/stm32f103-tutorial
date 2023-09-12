use super::conf::*;

use embedded_hal::digital::v2::{InputPin, OutputPin};

pub struct W25Q64<'a, SS, SCK, MOSI, MISO>
where
    SS: OutputPin,
    <SS as OutputPin>::Error: core::fmt::Debug,
    SCK: OutputPin,
    <SCK as OutputPin>::Error: core::fmt::Debug,
    MOSI: OutputPin,
    <MOSI as OutputPin>::Error: core::fmt::Debug,
    MISO: InputPin,
    <MISO as InputPin>::Error: core::fmt::Debug,
{
    ss: &'a mut SS,
    sck: &'a mut SCK,
    mosi: &'a mut MOSI,
    miso: &'a mut MISO,
}

impl<'a, SS, SCK, MOSI, MISO> W25Q64<'a, SS, SCK, MOSI, MISO>
where
    SS: OutputPin,
    <SS as OutputPin>::Error: core::fmt::Debug,
    SCK: OutputPin,
    <SCK as OutputPin>::Error: core::fmt::Debug,
    MOSI: OutputPin,
    <MOSI as OutputPin>::Error: core::fmt::Debug,
    MISO: InputPin,
    <MISO as InputPin>::Error: core::fmt::Debug,
{
    pub fn new(ss: &'a mut SS, sck: &'a mut SCK, mosi: &'a mut MOSI, miso: &'a mut MISO) -> Self {
        W25Q64 {
            ss,
            sck,
            mosi,
            miso,
        }
    }

    pub fn spi_w_ss(&mut self, bit_value: u8) {
        if bit_value > 0 {
            self.ss.set_high().unwrap();
        } else {
            self.ss.set_low().unwrap();
        }
    }

    pub fn spi_w_sck(&mut self, bit_value: u8) {
        if bit_value > 0 {
            self.sck.set_high().unwrap();
        } else {
            self.sck.set_low().unwrap();
        }
    }

    pub fn spi_w_mosi(&mut self, bit_value: u8) {
        if bit_value > 0 {
            self.mosi.set_high().unwrap();
        } else {
            self.mosi.set_low().unwrap();
        }
    }

    pub fn spi_r_miso(&mut self) -> u8 {
        if self.miso.is_high().unwrap() {
            1
        } else {
            0
        }
    }

    /// PSI 开始
    pub fn spi_start(&mut self) {
        self.spi_w_ss(0);
    }

    /// PSI 结束
    pub fn spi_stop(&mut self) {
        self.spi_w_ss(1);
    }

    /// SPI 交换字节
    pub fn spi_swap_byte(&mut self, byte_send: u8) -> u8 {
        let mut byte_receive = 0x00;

        for i in 0..8 {
            self.spi_w_mosi(byte_send & (0x80 >> i));
            self.spi_w_sck(1);

            if self.spi_r_miso() == 1 {
                byte_receive |= 0x80 >> i;
            }
            self.spi_w_sck(0);
        }

        byte_receive
    }

    /// SPI 初始化
    /// push pull output pin 4,5,7
    /// pulled up input pin 6
    pub fn init_spi(&mut self) {
        self.spi_w_ss(1);
        self.spi_w_sck(0);
    }

    /// 初始化 W25Q64
    pub fn init_w25q64(&mut self) {
        self.init_spi();
    }

    /// 读取 W25Q64 ID
    pub fn read_id(&mut self) -> (u8, u16) {
        self.spi_start();
        self.spi_swap_byte(W25Q64_JEDEC_ID);

        let mid = self.spi_swap_byte(W25Q64_DUMMY_BYTE);
        let mut did = self.spi_swap_byte(W25Q64_DUMMY_BYTE) as u16;
        did <<= 8;
        did |= self.spi_swap_byte(W25Q64_DUMMY_BYTE) as u16;
        self.spi_stop();

        (mid, did)
    }

    /// 启用写入功能
    pub fn write_enable(&mut self) {
        self.spi_start();
        self.spi_swap_byte(W25Q64_WRITE_ENABLE);
        self.spi_stop();
    }

    /// 等待W25Q64芯片空闲
    pub fn wait_busy(&mut self) {
        let mut timeout: u32;
        self.spi_start();
        self.spi_swap_byte(W25Q64_READ_STATUS_REGISTER_1);
        timeout = 100000;
        while self.spi_swap_byte(W25Q64_DUMMY_BYTE) & 0x01 == 0x01 {
            timeout -= 1;
            if timeout == 0 {
                break;
            }
        }
        self.spi_stop();
    }

    pub fn page_program(&mut self, address: u32, data_array: &[u8]) {
        self.write_enable();

        self.spi_start();
        self.spi_swap_byte(W25Q64_PAGE_PROGRAM);
        self.spi_swap_byte((address >> 16) as u8);
        self.spi_swap_byte((address >> 8) as u8);
        self.spi_swap_byte(address as u8);

        for data in data_array {
            self.spi_swap_byte(*data);
        }
        self.spi_stop();

        self.wait_busy();
    }

    /// 擦除地址所在的扇区
    pub fn sector_erase(&mut self, address: u32) {
        self.write_enable();

        self.spi_start();
        self.spi_swap_byte(W25Q64_SECTOR_ERASE_4KB);
        self.spi_swap_byte((address >> 16) as u8);
        self.spi_swap_byte((address >> 8) as u8);
        self.spi_swap_byte(address as u8);
        self.spi_stop();

        self.wait_busy();
    }

    /// 读取数据
    pub fn read_data(&mut self, address: u32, data_array: &mut [u8]) {
        self.spi_start();
        self.spi_swap_byte(W25Q64_READ_DATA);
        self.spi_swap_byte((address >> 16) as u8);
        self.spi_swap_byte((address >> 8) as u8);
        self.spi_swap_byte(address as u8);

        for item in data_array {
            *item = self.spi_swap_byte(W25Q64_DUMMY_BYTE);
        }
        self.spi_stop();
    }
}
