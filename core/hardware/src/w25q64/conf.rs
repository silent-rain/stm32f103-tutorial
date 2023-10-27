//! 配置
//! 定义W25Q64芯片的相关命令和参数

// 写使能命令
pub const W25Q64_WRITE_ENABLE: u8 = 0x06;
// 写禁止命令
pub const W25Q64_WRITE_DISABLE: u8 = 0x04;
// 读状态寄存器1命令
pub const W25Q64_READ_STATUS_REGISTER_1: u8 = 0x05;
pub const W25Q64_READ_STATUS_REGISTER_2: u8 = 0x35;
pub const W25Q64_WRITE_STATUS_REGISTER: u8 = 0x01;
// 页编程命令
pub const W25Q64_PAGE_PROGRAM: u8 = 0x02;
pub const W25Q64_QUAD_PAGE_PROGRAM: u8 = 0x32;
// 块擦除命令
pub const W25Q64_BLOCK_ERASE_64KB: u8 = 0xD8;
pub const W25Q64_BLOCK_ERASE_32KB: u8 = 0x52;
// 扇区擦除命令
pub const W25Q64_SECTOR_ERASE_4KB: u8 = 0x20;
pub const W25Q64_CHIP_ERASE: u8 = 0xC7;
pub const W25Q64_ERASE_SUSPEND: u8 = 0x75;
pub const W25Q64_ERASE_RESUME: u8 = 0x7A;
pub const W25Q64_POWER_DOWN: u8 = 0xB9;
pub const W25Q64_HIGH_PERFORMANCE_MODE: u8 = 0xA3;
pub const W25Q64_CONTINUOUS_READ_MODE_RESET: u8 = 0xFF;
pub const W25Q64_RELEASE_POWER_DOWN_HPM_DEVICE_ID: u8 = 0xAB;
// 读取芯片的制造商和设备ID
pub const W25Q64_MANUFACTURER_DEVICE_ID: u8 = 0x90;
pub const W25Q64_READ_UNIQUE_ID: u8 = 0x4B;
// 读取芯片的JEDEC设备ID
pub const W25Q64_JEDEC_DEVICE_ID: u8 = 0x9F;
// 读数据命令
pub const W25Q64_READ_DATA: u8 = 0x03;
pub const W25Q64_FAST_READ: u8 = 0x0B;
pub const W25Q64_FAST_READ_DUAL_OUTPUT: u8 = 0x3B;
pub const W25Q64_FAST_READ_DUAL_IO: u8 = 0xBB;
pub const W25Q64_FAST_READ_QUAD_OUTPUT: u8 = 0x6B;
pub const W25Q64_FAST_READ_QUAD_IO: u8 = 0xEB;
pub const W25Q64_OCTAL_WORD_READ_QUAD_IO: u8 = 0xE3;

pub const W25Q64_DUMMY_BYTE: u8 = 0xFF;

// 页大小为256字节
pub const W25Q64_PAGE_SIZE: usize = 256;
