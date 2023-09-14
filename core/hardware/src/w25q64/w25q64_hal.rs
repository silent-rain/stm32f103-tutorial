use super::conf::*;

use cortex_m::prelude::{_embedded_hal_blocking_spi_Transfer, _embedded_hal_blocking_spi_Write};
use embedded_hal::digital::v2::OutputPin;
use stm32f1xx_hal::pac::{self, SPI1};
use stm32f1xx_hal::prelude::_fugit_RateExtU32;
use stm32f1xx_hal::spi::{Master, Pins, Spi, Spi1NoRemap, SpiBitFormat};
use stm32f1xx_hal::{afio, rcc, spi};

pub struct W25Q64<'a, PINS, SS>
where
    PINS: Pins<Spi1NoRemap>,
    SS: OutputPin,
    <SS as OutputPin>::Error: core::fmt::Debug,
{
    spi: Spi<pac::SPI1, Spi1NoRemap, PINS, u8, Master>,
    ss: &'a mut SS,
}

impl<'a, PINS, SS> W25Q64<'a, PINS, SS>
where
    PINS: Pins<Spi1NoRemap>,
    SS: OutputPin,
    <SS as OutputPin>::Error: core::fmt::Debug,
{
    pub fn new(
        spi1: SPI1,
        pins: PINS,
        ss: &'a mut SS,
        mapr: &'a mut afio::MAPR,
        clocks: rcc::Clocks,
    ) -> Self {
        let mode = spi::Mode {
            polarity: spi::Polarity::IdleLow,
            phase: spi::Phase::CaptureOnFirstTransition,
        };

        // 创建一个Spi实例
        let mut spi = Spi::spi1(spi1, pins, mapr, mode, 1.MHz(), clocks);
        // 选择用于数据传输的帧格式
        spi.bit_format(SpiBitFormat::MsbFirst);

        let mut w25q = W25Q64 { spi, ss };
        w25q.spi_w_ss(1);
        w25q
    }

    pub fn spi_w_ss(&mut self, bit_value: u8) {
        if bit_value == 0 {
            // 拉低片选信号，开始通信
            self.ss.set_low().unwrap();
        } else {
            // 拉高片选信号，结束通信
            self.ss.set_high().unwrap();
        }
    }

    pub fn spi_start(&mut self) {
        self.spi_w_ss(0);
    }

    pub fn spi_stop(&mut self) {
        self.spi_w_ss(1);
    }

    /// 写入数据
    pub fn spi_write(&mut self, words: &[u8]) -> Result<(), spi::Error> {
        while !self.spi.is_tx_empty() {}
        self.spi.write(words)?;
        while self.spi.is_rx_not_empty() {}
        Ok(())
    }

    /// 需写入并返回数据
    pub fn spi_transfer(&mut self, words: &mut [u8]) -> Result<(), spi::Error> {
        while !self.spi.is_tx_empty() {}
        self.spi.transfer(words)?;
        while self.spi.is_rx_not_empty() {}
        Ok(())
    }

    /// 启用写入功能
    pub fn write_enable(&mut self) -> Result<(), spi::Error> {
        self.spi_start();
        // 发送写使能命令
        self.spi_write(&[W25Q64_WRITE_ENABLE])?;
        self.spi_stop();
        Ok(())
    }

    /// 禁用写入功能
    pub fn write_disable(&mut self) -> Result<(), spi::Error> {
        // 禁用写入功能
        self.spi_start();
        // 发送写禁止命令
        self.spi_write(&[W25Q64_WRITE_DISABLE])?;
        self.spi_stop();

        Ok(())
    }

    /// 读取W25Q64芯片的 JEDEC ID
    /// 使用Spi实例和片选引脚来发送和接收命令和数据
    /// 0xEF4017: 代表W25Q64芯片
    pub fn read_jedec_id(&mut self) -> Result<(u8, u16), spi::Error> {
        self.spi_start();
        self.spi_write(&[W25Q64_JEDEC_ID]).unwrap();

        let mut buffer = [W25Q64_DUMMY_BYTE; 3];
        self.spi_transfer(&mut buffer).unwrap();
        self.spi_stop();

        let mid = buffer[0];
        let did = (buffer[1] as u16) << 8 | buffer[2] as u16;
        // EF, 4017
        Ok((mid, did))
    }

    /// 读取W25Q64芯片的制造商设备ID
    ///
    /// 使用Spi实例和片选引脚来发送和接收命令和数据
    /// 0xEF16: 代表W25Q64芯片
    pub fn read_device_id(&mut self) -> Result<u16, spi::Error> {
        let cmd = [W25Q64_MANUFACTURER_DEVICE_ID, 0, 0, 0, 0, 0];
        let mut buffer = [0; 6];

        self.spi_start();
        self.spi.write(&cmd).unwrap();
        self.spi.transfer(&mut buffer).unwrap();
        self.spi_stop();

        let device_id = (buffer[0] as u16) << 8 | buffer[1] as u16;
        Ok(device_id)
    }

    /// 读取状态寄存器1
    pub fn read_status_register_1(&mut self) -> Result<u8, spi::Error> {
        let mut buffer = [W25Q64_READ_STATUS_REGISTER_1, 0];
        self.spi_start();
        self.spi_transfer(&mut buffer)?;
        self.spi_stop();
        Ok(buffer[1])
    }

    /// 检查是否有写保护标志
    pub fn check_write_protect(&mut self) -> Result<bool, spi::Error> {
        let status = self.read_status_register_1()?;
        let srp0 = status & 0x80;
        let srp1 = status & 0x04;
        if srp0 == 0 && srp1 == 0 {
            // 没有写保护
            Ok(false)
        } else {
            // 有写保护
            Ok(true)
        }
    }

    /// 定义一个辅助函数，用于等待W25Q64芯片空闲
    pub fn wait_for_idle2(&mut self) -> Result<(), spi::Error> {
        self.spi_start();
        self.spi.write(&[W25Q64_READ_STATUS_REGISTER_1])?; // 发送读状态寄存器1命令
        let mut status = [0x00; 1];
        let mut timeout = 100000;
        loop {
            self.spi.transfer(&mut status)?; // 接收状态寄存器1的值
            if status[0] & 0x01 == 0 {
                // 检查状态寄存器1的最低位，如果为0表示空闲，否则表示忙碌
                break;
            }
            timeout -= 1;
            if timeout == 0 {
                break;
            }
        }
        self.spi_stop();
        Ok(())
    }

    /// 定义一个辅助函数，用于等待W25Q64芯片空闲
    pub fn wait_for_idle(&mut self) -> Result<(), spi::Error> {
        self.spi_start();
        let mut timeout = 100000;
        // 空闲时退出
        while self.spi.is_busy() {
            timeout -= 1;
            if timeout == 0 {
                break;
            }
        }
        self.spi_stop();
        Ok(())
    }

    /// 页编程, 写入数据
    /// page_address: 设定页地址
    /// data: 要写入的数据
    pub fn page_program(&mut self, page_address: u32, data: &[u8]) -> Result<(), spi::Error> {
        self.write_enable().unwrap();

        let cmd = [
            W25Q64_PAGE_PROGRAM,
            (page_address >> 16) as u8,
            (page_address >> 8) as u8,
            page_address as u8,
        ];
        self.spi_start();
        self.spi_write(&cmd).unwrap();
        self.spi_write(data).unwrap();
        self.spi_stop();

        // 等待W25Q64芯片空闲
        self.wait_for_idle()?;
        Ok(())
    }

    /// 擦除地址所在的扇区
    pub fn sector_erase(&mut self, address: u32) -> Result<(), spi::Error> {
        self.write_enable().unwrap();

        let cmd = [
            W25Q64_SECTOR_ERASE_4KB,
            (address >> 16) as u8,
            (address >> 8) as u8,
            address as u8,
        ];
        self.spi_start();
        self.spi_write(&cmd)?;
        self.spi_stop();

        self.wait_for_idle()?;
        Ok(())
    }

    /// 读取数据
    /// read_address: 目标地址
    /// data: 用于存放数据
    pub fn read_data(&mut self, read_address: u32, data: &mut [u8]) -> Result<(), spi::Error> {
        let cmd = [
            W25Q64_READ_DATA,
            (read_address >> 16) as u8,
            (read_address >> 8) as u8,
            read_address as u8,
            // W25Q64_DUMMY_BYTE, // 发送一个虚拟字节
        ];

        self.spi_start();
        self.spi_write(&cmd).unwrap();

        // 使用fill方法来赋值为虚拟字节
        data.fill(W25Q64_DUMMY_BYTE);

        // 接收请求的数据
        self.spi_transfer(data).unwrap();
        self.spi_stop();

        // 调整数据字节的顺序，高位在前，低位在后
        // data.reverse();

        Ok(())
    }
}
