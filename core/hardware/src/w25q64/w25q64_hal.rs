use super::conf::*;

use cortex_m::prelude::{_embedded_hal_blocking_spi_Transfer, _embedded_hal_blocking_spi_Write};
use embedded_hal::digital::v2::OutputPin;
use stm32f1xx_hal::pac::{self, SPI1};
use stm32f1xx_hal::prelude::_fugit_RateExtU32;
use stm32f1xx_hal::spi::{Master, Pins, Spi, Spi1NoRemap};
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
    /// 初始化 W25Q64
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
        let spi = Spi::spi1(spi1, pins, mapr, mode, 1.MHz(), clocks);
        W25Q64 { spi, ss }
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

    /// 启用写入功能
    pub fn write_enable(&mut self) -> Result<(), spi::Error> {
        self.spi_start();
        // 发送写使能命令
        self.spi.write(&[W25Q64_WRITE_ENABLE])?;
        self.spi_stop();
        Ok(())
    }

    /// 禁用写入功能
    pub fn write_disable(&mut self) -> Result<(), spi::Error> {
        // 禁用写入功能
        self.spi_start();
        // 发送写禁止命令
        self.spi.write(&[W25Q64_WRITE_DISABLE])?;
        self.spi_stop();

        Ok(())
    }

    /// 读取W25Q64芯片的 JEDEC ID
    /// 使用Spi实例和片选引脚来发送和接收命令和数据
    pub fn read_jedec_id(&mut self) -> Result<u8, spi::Error> {
        self.spi_start();
        self.spi.write(&[W25Q64_JEDEC_ID]).unwrap();

        let mut buffer = [W25Q64_DUMMY_BYTE; 3];
        self.spi.transfer(&mut buffer).unwrap();
        self.spi_stop();

        // 处理读取到的 JEDEC ID（这里您可以根据需要进行操作）
        // println!("Manufacturer ID: {:X}", buffer[0]);
        // println!("Memory Type: {:X}", buffer[1]);
        // println!("Capacity: {:X}", buffer[2]);
        let jedec_id = buffer[0];
        Ok(jedec_id)
    }

    /// 读取W25Q64芯片的MID和DID
    ///
    /// 使用Spi实例和片选引脚来发送和接收命令和数据
    pub fn read_device_id(&mut self) -> Result<(u8, u16), spi::Error> {
        let cmd = [W25Q64_MANUFACTURER_DEVICE_ID, 0, 0, 0, 0, 0];
        let mut buffer = [W25Q64_DUMMY_BYTE; 6];

        self.spi_start();
        self.spi.write(&cmd).unwrap();
        self.spi.transfer(&mut buffer).unwrap();
        self.spi_stop();

        let mid = buffer[0];
        let did = (buffer[1] as u16) << 8 | buffer[2] as u16;
        // EF, 4017
        Ok((mid, did))
    }

    /// 定义一个辅助函数，用于等待W25Q64芯片空闲
    pub fn wait_for_idle(&mut self) -> Result<(), spi::Error> {
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
        self.spi.write(&cmd).unwrap();
        self.spi.write(data).unwrap();
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
        self.spi.write(&cmd)?;
        self.spi_stop();

        // 等待W25Q64芯片空闲
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
        ];

        self.spi_start();
        self.spi.write(&cmd).unwrap();

        // 接收请求的数据
        self.spi.transfer(data).unwrap();
        self.spi_stop();

        Ok(())
    }
}
