use super::conf::*;

use embedded_hal::digital::v2::{InputPin, OutputPin};

/// PSI 开始
pub fn spi_start<SS>(ss: &mut SS)
where
    SS: OutputPin,
{
    let _ = ss.set_low();
}

/// PSI 结束
pub fn spi_stop<SS>(ss: &mut SS)
where
    SS: OutputPin,
{
    let _ = ss.set_high();
}

/// SPI 交换字节
pub fn spi_swap_byte<SCK, MOSI, MISO>(
    sck: &mut SCK,
    mosi: &mut MOSI,
    miso: &mut MISO,
    byte_send: u8,
) -> u8
where
    SCK: OutputPin,
    MOSI: OutputPin,
    MISO: InputPin,
    <MISO as InputPin>::Error: core::fmt::Debug,
{
    let mut byte_receive = 0x00;

    for i in 0..8 {
        if byte_send & (0x80 >> i) > 0 {
            let _ = mosi.set_high();
        } else {
            let _ = mosi.set_low();
        }

        let _ = sck.set_high();

        if miso.is_high().unwrap() {
            byte_receive |= 0x80 >> i;
        }
        let _ = sck.set_low();
    }

    byte_receive
}

/// SPI 初始化
/// push pull output pin 4,5,7
/// pulled up input pin 6
pub fn init_spi<SS, SCK>(ss: &mut SS, sck: &mut SCK)
where
    SS: OutputPin,
    SCK: OutputPin,
{
    let _ = ss.set_high();
    let _ = sck.set_low();
}

/// 初始化 W25Q64
pub fn init_w25q64<SS, SCK>(ss: &mut SS, sck: &mut SCK)
where
    SS: OutputPin,
    SCK: OutputPin,
{
    init_spi(ss, sck);
}

/// 读取 W25Q64 ID
pub fn read_id<SS, SCK, MOSI, MISO>(
    ss: &mut SS,
    sck: &mut SCK,
    mosi: &mut MOSI,
    miso: &mut MISO,
) -> (u8, u16)
where
    SS: OutputPin,
    SCK: OutputPin,
    MOSI: OutputPin,
    MISO: InputPin,
    <MISO as InputPin>::Error: core::fmt::Debug,
{
    spi_start(ss);
    spi_swap_byte(sck, mosi, miso, W25Q64_JEDEC_ID);

    let mid = spi_swap_byte(sck, mosi, miso, W25Q64_DUMMY_BYTE);
    let mut did = spi_swap_byte(sck, mosi, miso, W25Q64_DUMMY_BYTE) as u16;
    did <<= 8;
    did |= spi_swap_byte(sck, mosi, miso, W25Q64_DUMMY_BYTE) as u16;
    spi_stop(ss);

    (mid, did)
}

/// 启用写入功能
pub fn write_enable<SS, SCK, MOSI, MISO>(
    ss: &mut SS,
    sck: &mut SCK,
    mosi: &mut MOSI,
    miso: &mut MISO,
) where
    SS: OutputPin,
    SCK: OutputPin,
    MOSI: OutputPin,
    MISO: InputPin,
    <MISO as InputPin>::Error: core::fmt::Debug,
{
    spi_start(ss);
    spi_swap_byte(sck, mosi, miso, W25Q64_WRITE_ENABLE);
    spi_stop(ss);
}

/// 等待W25Q64芯片空闲
pub fn wait_busy<SS, SCK, MOSI, MISO>(ss: &mut SS, sck: &mut SCK, mosi: &mut MOSI, miso: &mut MISO)
where
    SS: OutputPin,
    SCK: OutputPin,
    MOSI: OutputPin,
    MISO: InputPin,
    <MISO as InputPin>::Error: core::fmt::Debug,
{
    let mut timeout: u32;
    spi_start(ss);
    spi_swap_byte(sck, mosi, miso, W25Q64_READ_STATUS_REGISTER_1);
    timeout = 100000;
    while spi_swap_byte(sck, mosi, miso, W25Q64_DUMMY_BYTE) & 0x01 == 0x01 {
        timeout -= 1;
        if timeout == 0 {
            break;
        }
    }
    spi_stop(ss);
}

pub fn page_program<SS, SCK, MOSI, MISO>(
    ss: &mut SS,
    sck: &mut SCK,
    mosi: &mut MOSI,
    miso: &mut MISO,
    address: u32,
    data_array: &[u8],
) where
    SS: OutputPin,
    SCK: OutputPin,
    MOSI: OutputPin,
    MISO: InputPin,
    <MISO as InputPin>::Error: core::fmt::Debug,
{
    write_enable(ss, sck, mosi, miso);

    spi_start(ss);
    spi_swap_byte(sck, mosi, miso, W25Q64_PAGE_PROGRAM);
    spi_swap_byte(sck, mosi, miso, (address >> 16) as u8);
    spi_swap_byte(sck, mosi, miso, (address >> 8) as u8);
    spi_swap_byte(sck, mosi, miso, address as u8);

    for (i, _) in data_array.iter().enumerate() {
        spi_swap_byte(sck, mosi, miso, data_array[i]);
    }
    spi_stop(ss);

    wait_busy(ss, sck, mosi, miso);
}

/// 擦除地址所在的扇区
pub fn sector_erase<SS, SCK, MOSI, MISO>(
    ss: &mut SS,
    sck: &mut SCK,
    mosi: &mut MOSI,
    miso: &mut MISO,
    address: u32,
) where
    SS: OutputPin,
    SCK: OutputPin,
    MOSI: OutputPin,
    MISO: InputPin,
    <MISO as InputPin>::Error: core::fmt::Debug,
{
    write_enable(ss, sck, mosi, miso);

    spi_start(ss);
    spi_swap_byte(sck, mosi, miso, W25Q64_SECTOR_ERASE_4KB);
    spi_swap_byte(sck, mosi, miso, (address >> 16) as u8);
    spi_swap_byte(sck, mosi, miso, (address >> 8) as u8);
    spi_swap_byte(sck, mosi, miso, address as u8);
    spi_stop(ss);

    wait_busy(ss, sck, mosi, miso);
}

/// 读取数据
pub fn read_data<SS, SCK, MOSI, MISO>(
    ss: &mut SS,
    sck: &mut SCK,
    mosi: &mut MOSI,
    miso: &mut MISO,
    address: u32,
    data_array: &mut [u8],
) where
    SS: OutputPin,
    SCK: OutputPin,
    MOSI: OutputPin,
    MISO: InputPin,
    <MISO as InputPin>::Error: core::fmt::Debug,
{
    spi_start(ss);
    spi_swap_byte(sck, mosi, miso, W25Q64_READ_DATA);
    spi_swap_byte(sck, mosi, miso, (address >> 16) as u8);
    spi_swap_byte(sck, mosi, miso, (address >> 8) as u8);
    spi_swap_byte(sck, mosi, miso, address as u8);

    for item in data_array {
        *item = spi_swap_byte(sck, mosi, miso, W25Q64_DUMMY_BYTE);
    }
    spi_stop(ss);
}
