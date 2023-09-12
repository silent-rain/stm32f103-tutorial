use super::conf::*;

use cortex_m::prelude::{_embedded_hal_blocking_spi_Transfer, _embedded_hal_blocking_spi_Write};
use stm32f1xx_hal::gpio::{Output, PushPull, PA4};
use stm32f1xx_hal::pac::{self, SPI1};
use stm32f1xx_hal::prelude::_fugit_RateExtU32;
use stm32f1xx_hal::spi::{Master, Pins, Remap, Spi, Spi1NoRemap};
use stm32f1xx_hal::{afio, rcc, spi};

/// 初始化 W25Q64
pub fn init_w25q64<PINS>(
    spi1: SPI1,
    pins: PINS,
    mapr: &mut afio::MAPR,
    clocks: rcc::Clocks,
) -> Spi<pac::SPI1, Spi1NoRemap, PINS, u8, Master>
where
    PINS: Pins<Spi1NoRemap>,
{
    let mode = spi::Mode {
        polarity: spi::Polarity::IdleLow,
        phase: spi::Phase::CaptureOnFirstTransition,
    };

    // 创建一个Spi实例
    Spi::spi1(spi1, pins, mapr, mode, 1.MHz(), clocks)
}

/// 启用写入功能
pub fn write_enable<REMAP, PINS>(
    spi: &mut Spi<pac::SPI1, REMAP, PINS, u8, Master>,
    cs: &mut PA4<Output<PushPull>>,
) -> Result<(), spi::Error>
where
    REMAP: Remap<Periph = pac::SPI1>,
    PINS: Pins<REMAP>,
{
    // 启用写入功能
    cs.set_low(); // 拉低片选信号，开始通信
    spi.write(&[W25Q64_WRITE_ENABLE])?; // 发送写使能命令
    cs.set_high(); // 拉高片选信号，结束通信

    Ok(())
}

/// 禁用写入功能
pub fn write_disable<REMAP, PINS>(
    spi: &mut Spi<pac::SPI1, REMAP, PINS, u8, Master>,
    cs: &mut PA4<Output<PushPull>>,
) -> Result<(), spi::Error>
where
    REMAP: Remap<Periph = pac::SPI1>,
    PINS: Pins<REMAP>,
{
    // 禁用写入功能
    cs.set_low(); // 拉低片选信号，开始通信
    spi.write(&[W25Q64_WRITE_DISABLE])?; // 发送写禁止命令
    cs.set_high(); // 拉高片选信号，结束通信

    Ok(())
}

/// 读取W25Q64芯片的 JEDEC ID
/// 使用Spi实例和片选引脚来发送和接收命令和数据
pub fn read_jedec_id<REMAP, PINS>(
    spi: &mut Spi<pac::SPI1, REMAP, PINS, u8, Master>,
    cs: &mut PA4<Output<PushPull>>,
) -> Result<u64, spi::Error>
where
    REMAP: Remap<Periph = pac::SPI1>,
    PINS: Pins<REMAP>,
{
    let mut buffer = [0x00; 5]; // 缓冲区，用于存放命令和数据，大小为5个字节
    buffer[0] = W25Q64_JEDEC_ID; // 第一个字节为读ID命令
    cs.set_low(); // 拉低片选信号，开始通信
    spi.transfer(&mut buffer)?; // 发送并接收数据，覆盖缓冲区
    cs.set_high(); // 拉高片选信号，结束通信

    // 从缓冲区中提取读取到的ID
    // 从第二到第五个字节为读取到的ID，使用大端格式，并补齐为64位整数
    let id = u64::from_be_bytes([
        buffer[1], buffer[2], buffer[3], buffer[4], 0x00, 0x00, 0x00, 0x00,
    ]);

    Ok(id)
}

/// 读取W25Q64芯片的MID和DID
///
/// 使用Spi实例和片选引脚来发送和接收命令和数据
pub fn read_device_id<REMAP, PINS>(
    spi: &mut Spi<pac::SPI1, REMAP, PINS, u8, Master>,
    cs: &mut PA4<Output<PushPull>>,
) -> Result<(u8, u8), spi::Error>
where
    REMAP: Remap<Periph = pac::SPI1>,
    PINS: Pins<REMAP>,
{
    let mut buffer = [0x00; 7]; // 缓冲区，用于存放命令和数据，大小为6个字节
    buffer[0] = W25Q64_MANUFACTURER_DEVICE_ID; // 第一个字节为读MID和DID命令
    buffer[1..5].copy_from_slice(&0x000000_u32.to_be_bytes()); // 第二到第四个字节为固定地址0x000000，使用大端格式

    cs.set_low(); // 拉低片选信号，开始通信
    spi.transfer(&mut buffer)?; // 发送并接收数据，覆盖缓冲区
    cs.set_high(); // 拉高片选信号，结束通信

    // 从缓冲区中提取读取到的MID和DID
    let mid = buffer[5]; // 第五个字节为读取到的MID
    let did = buffer[6]; // 第六个字节为读取到的DID

    // EF, 4017
    Ok((mid, did))
}

/// 定义一个辅助函数，用于等待W25Q64芯片空闲
pub fn wait_for_idle<REMAP, PINS>(
    spi: &mut Spi<pac::SPI1, REMAP, PINS, u8, Master>,
    cs: &mut PA4<Output<PushPull>>,
) -> Result<(), spi::Error>
where
    REMAP: Remap<Periph = pac::SPI1>,
    PINS: Pins<REMAP>,
{
    let mut status = [0x00];
    loop {
        cs.set_low(); // 拉低片选信号，开始通信
        spi.write(&[W25Q64_READ_STATUS_REGISTER_1])?; // 发送读状态寄存器1命令
        spi.transfer(&mut status)?; // 接收状态寄存器1的值
        cs.set_high(); // 拉高片选信号，结束通信
        if status[0] & 0x01 == 0 {
            // 检查状态寄存器1的最低位，如果为0表示空闲，否则表示忙碌
            break;
        }
    }
    Ok(())
}

/// 页编程, 写入数据
pub fn page_program<REMAP, PINS>(
    spi: &mut Spi<pac::SPI1, REMAP, PINS, u8, Master>,
    cs: &mut PA4<Output<PushPull>>,
    address: u32, // 目标地址
    data: &[u8],  // 要写入的数据
) -> Result<(), spi::Error>
where
    REMAP: Remap<Periph = pac::SPI1>,
    PINS: Pins<REMAP>,
{
    // 缓冲区，4个字节的命令和地址
    let mut cmd_buffer = [0x00; 5];
    cmd_buffer[0] = W25Q64_PAGE_PROGRAM; // 第一个字节为页编程命令
    cmd_buffer[1..5].copy_from_slice(&address.to_be_bytes()); // 第二到第四个字节为目标地址，使用大端格式

    cs.set_low(); // 拉低片选信号，开始通信
    spi.write(&cmd_buffer)?; // 发送缓冲区中的命令
    spi.write(data)?; // 要写入的数据

    cs.set_high(); // 拉高片选信号，结束通信

    // 等待W25Q64芯片空闲
    wait_for_idle(spi, cs)?;
    Ok(())
}

/// 读取数据
pub fn read_data<REMAP, PINS>(
    spi: &mut Spi<pac::SPI1, REMAP, PINS, u8, Master>,
    cs: &mut PA4<Output<PushPull>>,
    address: u32,      // 目标地址
    buffer: &mut [u8], // 缓冲区，用于存放数据
) -> Result<(), spi::Error>
where
    REMAP: Remap<Periph = pac::SPI1>,
    PINS: Pins<REMAP>,
{
    let mut cmd_buffer = [0x00; 5]; // 缓冲区，用于存放命令，大小为4个字节
    cmd_buffer[0] = W25Q64_READ_DATA; // 第一个字节为读数据命令
    cmd_buffer[1..5].copy_from_slice(&address.to_be_bytes()); // 第二到第四个字节为目标地址，使用大端格式

    cs.set_low(); // 拉低片选信号，开始通信
    spi.write(&cmd_buffer)?; // 发送缓冲区中的命令
    spi.transfer(buffer)?; // 发送并接收数据，覆盖缓冲区
    cs.set_high(); // 拉高片选信号，结束通信

    Ok(())
}

/// 擦除地址所在的扇区
pub fn sector_erase<REMAP, PINS>(
    spi: &mut Spi<pac::SPI1, REMAP, PINS, u8, Master>,
    cs: &mut PA4<Output<PushPull>>,
    address: u32, // 目标地址
) -> Result<(), spi::Error>
where
    REMAP: Remap<Periph = pac::SPI1>,
    PINS: Pins<REMAP>,
{
    let mut buffer = [0x00; 5]; // 缓冲区，用于存放命令和地址，大小为4个字节
    buffer[0] = W25Q64_SECTOR_ERASE_4KB; // 第一个字节为Sector Erase命令
    buffer[1..5].copy_from_slice(&address.to_be_bytes()); // 第二到第四个字节为目标地址，使用大端格式

    cs.set_low(); // 拉低片选信号，开始通信
    spi.write(&buffer)?; // 发送缓冲区中的命令和地址
    cs.set_high(); // 拉高片选信号，结束通信

    // 等待W25Q64芯片空闲
    wait_for_idle(spi, cs)?;
    Ok(())
}
