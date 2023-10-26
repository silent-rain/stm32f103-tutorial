use core::ptr::{read_volatile, write_volatile};

use stm32f1xx_hal::pac::FLASH;

// FLASH Keys
// #define FLASH_KEY1               ((uint32_t)0x45670123)
// #define FLASH_KEY2               ((uint32_t)0xCDEF89AB)
const FLASH_KEY1: u32 = 0x45670123;
const FLASH_KEY2: u32 = 0xCDEF89AB;

pub struct FlashStore<'a> {
    pub flash: &'a FLASH,
}

impl<'a> FlashStore<'a> {
    pub fn new() -> Self {
        let flash = unsafe { (FLASH::ptr() as *mut FLASH).as_ref().unwrap() };
        FlashStore { flash }
    }

    /// FLASH读取一个32位的字
    /// address: 要读取数据的字地址
    pub fn flash_read_word(address: u32) -> u32 {
        // 使用指针访问指定地址下的数据并返回
        unsafe { read_volatile(address as *const u32) }
    }

    /// FLASH读取一个16位的半字
    /// address 要读取数据的半字地址
    pub fn flash_read_half_word(address: u32) -> u16 {
        // 使用指针访问指定地址下的数据并返回
        unsafe { read_volatile(address as *const u16) }
    }

    /// FLASH读取一个8位的字节
    /// address: 要读取数据的字节地址
    pub fn flash_read_byte(address: u32) -> u8 {
        // 使用指针访问指定地址下的数据并返回
        unsafe { read_volatile(address as *const u8) }
    }

    /// 解锁Flash
    fn unlock_flash(&self) {
        self.flash.acr.modify(|_, w| w.prftbe().set_bit());
        self.flash.keyr.write(|w| unsafe { w.bits(FLASH_KEY1) });
        self.flash.keyr.write(|w| unsafe { w.bits(FLASH_KEY2) });
    }

    /// 加锁Flash
    fn lock_flash(&self) {
        self.flash.cr.modify(|_, w| w.lock().set_bit());
    }

    /// 等待芯片空闲
    fn wait_busy(&self) {
        while self.flash.sr.read().bsy().bit_is_set() {}
    }

    /// FLASH全擦除
    /// 调用此函数后，FLASH的所有页都会被擦除，包括程序文件本身，擦除后，程序将不复存在
    pub fn flash_erase_all_pages(&self) {
        // 解锁Flash
        self.unlock_flash();

        // 全擦除Flash
        self.flash.cr.modify(|_, w| w.mer().set_bit());
        self.flash.cr.modify(|_, w| w.strt().set_bit());

        // 等待擦除完成
        self.wait_busy();

        // 加锁Flash
        self.lock_flash();
    }

    /// FLASH页擦除
    /// page_address: 要擦除页的页地址
    pub fn flash_erase_page(&self, page_address: u32) {
        // 解锁Flash
        self.unlock_flash();

        // 擦除指定页
        self.flash.cr.modify(|_, w| w.per().set_bit());
        self.flash.ar.write(|w| unsafe { w.bits(page_address) });
        self.flash.cr.modify(|_, w| w.strt().set_bit());

        // 等待擦除完成
        self.wait_busy();

        // 加锁Flash
        self.flash.cr.modify(|_, w| w.lock().set_bit());
    }

    /// FLASH编程字
    /// address: 要写入数据的字地址
    /// data: 要写入的32位数据
    pub fn flash_program_word(&self, address: u32, data: u32) {
        // 解锁Flash
        self.unlock_flash();

        // 编程字
        self.flash.cr.modify(|_, w| w.pg().set_bit());
        unsafe { write_volatile(address as *mut u16, (data & 0xFFFF) as u16) };

        // 等待编程完成
        self.wait_busy();

        unsafe { write_volatile((address + 2) as *mut u16, (data >> 16) as u16) };

        // 等待编程完成
        self.wait_busy();

        // 加锁Flash
        self.flash.cr.modify(|_, w| w.lock().set_bit());
    }

    /// FLASH编程半字
    /// address: 要写入数据的半字地址
    /// data: 要写入的16位数据
    pub fn flash_program_half_word(&self, address: u32, data: u16) {
        // 解锁Flash
        self.unlock_flash();

        // 编程半字
        self.flash.cr.modify(|_, w| w.pg().set_bit());
        unsafe { write_volatile(address as *mut u16, data) };

        // 等待编程完成
        self.wait_busy();

        // 加锁Flash
        self.flash.cr.modify(|_, w| w.lock().set_bit());
    }
}

impl<'a> Default for FlashStore<'a> {
    fn default() -> Self {
        Self::new()
    }
}
