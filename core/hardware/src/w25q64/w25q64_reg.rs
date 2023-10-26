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
        let mut w25q = W25Q64 {
            ss,
            sck,
            mosi,
            miso,
        };
        // SS 默认高电平
        w25q.spi_w_ss(1);
        // SCK 默认低电平
        w25q.spi_w_sck(0);
        w25q
    }

    pub fn spi_w_ss(&mut self, bit_value: u8) {
        if bit_value == 0 {
            self.ss.set_low().unwrap();
        } else {
            self.ss.set_high().unwrap();
        }
    }

    pub fn spi_w_sck(&mut self, bit_value: u8) {
        if bit_value == 0 {
            self.sck.set_low().unwrap();
        } else {
            self.sck.set_high().unwrap();
        }
    }

    pub fn spi_w_mosi(&mut self, bit_value: u8) {
        if bit_value == 0 {
            self.mosi.set_low().unwrap();
        } else {
            self.mosi.set_high().unwrap();
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

    /// SPI 交换传输一个字节，使用 SPI 模式 0
    /// ByteSend 要发送的一个字节
    /// 返回接收的一个字节
    pub fn spi_swap_byte(&mut self, byte_send: u8) -> u8 {
        // 定义接收的数据，并赋初值 0x00，此处必须赋初值 0x00，后面会用到
        let mut byte_receive = 0x00;

        // 循环 8 次，依次交换每一位数据
        for i in 0..8 {
            // 使用掩码的方式取出 ByteSend 的指定一位数据并写入到 MOSI 线
            self.spi_w_mosi(byte_send & (0x80 >> i));
            // 拉高 SCK，上升沿移出数据
            self.spi_w_sck(1);

            // 读取 MISO 数据，并存储到 Byte 变量
            // 当 MISO 为 1 时，置变量指定位为 1，当 MISO 为 0 时，不做处理，指定位为默认的初值 0
            if self.spi_r_miso() == 1 {
                byte_receive |= 0x80 >> i;
            }

            // 拉低 SCK，下降沿移入数据
            self.spi_w_sck(0);
        }

        // 返回接收到的一个字节数据
        byte_receive
    }

    /// 写入并返回数据
    pub fn spi_swap_bytes(&mut self, bytes: &mut [u8]) {
        self.spi_start();
        for byte in bytes {
            *byte = self.spi_swap_byte(*byte);
        }
        self.spi_stop();
    }

    /// 读取 W25Q64 ID
    pub fn read_id(&mut self) -> (u8, u16) {
        self.spi_start();
        self.spi_swap_byte(W25Q64_JEDEC_DEVICE_ID);

        let mid = self.spi_swap_byte(W25Q64_DUMMY_BYTE);
        let mut did = self.spi_swap_byte(W25Q64_DUMMY_BYTE) as u16; // 64
        did <<= 8;
        did |= self.spi_swap_byte(W25Q64_DUMMY_BYTE) as u16; //  23
        self.spi_stop();

        (mid, did)
    }

    /// 读取芯片的JEDEC设备ID
    /// 使用Spi实例和片选引脚来发送和接收命令和数据
    pub fn read_jedec_device_id(&mut self) -> (u8, u8, u8) {
        let mut buffer = [0; 4];
        buffer[0] = W25Q64_JEDEC_DEVICE_ID;
        self.spi_swap_bytes(&mut buffer);

        let manufacturer_id = buffer[1];
        let memory_type = buffer[2];
        let capacity = buffer[3];
        (manufacturer_id, memory_type, capacity)
    }

    /// 读取芯片的制造商和设备ID
    ///
    /// 使用Spi实例和片选引脚来发送和接收命令和数据
    /// 0xEF16: 代表W25Q64芯片
    pub fn read_manufacturer_device_id(&mut self) -> (u16, u16) {
        let mut buffer = [0; 7];
        buffer[0] = W25Q64_MANUFACTURER_DEVICE_ID;
        // 发送读取制造商和设备ID的命令
        self.spi_swap_bytes(&mut buffer);

        let manufacturer_id = buffer[4] as u16;
        let device_id = (buffer[5] as u16) << 8 | buffer[6] as u16;
        (manufacturer_id, device_id)
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

    /// 分页写入数据
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
