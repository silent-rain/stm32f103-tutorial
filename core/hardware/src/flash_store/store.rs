use super::FlashStore;

/// 存储的起始地址
const STORE_START_ADDRESS: u32 = 0x0800FC00;
/// 存储数据的个数
// const STORE_COUNT: usize = 512;

/// 定义SRAM数组
static mut STORE_DATA: [u16; 512] = [0; 512];

impl<'a> FlashStore<'a> {
    /// 设置存储内容
    pub fn set_store(&self, i: usize, value: u16) {
        unsafe {
            STORE_DATA[i] = value;
        }
    }

    /// 获取存储内容
    pub fn get_store(&self, i: usize) -> u16 {
        unsafe { STORE_DATA[i] }
    }

    /// 定义参数存储模块初始化函数
    pub fn init_store(&self) {
        // 判断是不是第一次使用
        // 读取第一个半字的标志位，if成立，则执行第一次使用的初始化
        if FlashStore::flash_read_half_word(STORE_START_ADDRESS) != 0xA5A5 {
            // 擦除指定页
            self.flash_erase_page(STORE_START_ADDRESS);
            // 在第一个半字写入自己规定的标志位，用于判断是不是第一次使用
            self.flash_program_half_word(STORE_START_ADDRESS, 0xA5A5);

            // 循环STORE_COUNT次，除了第一个标志位
            for (i, _) in unsafe { STORE_DATA[1..].iter_mut().enumerate() } {
                // 除了标志位的有效数据全部清0
                let address = STORE_START_ADDRESS + i as u32 * 2;
                self.flash_program_half_word(address, 0x0000);
            }
        }

        // 上电时，将闪存数据加载回SRAM数组，实现SRAM数组的掉电不丢失
        // 循环STORE_COUNT次，包括第一个标志位
        for (i, data) in unsafe { STORE_DATA.iter_mut().enumerate() } {
            // 将闪存的数据加载回SRAM数组
            let address = STORE_START_ADDRESS + i as u32 * 2;
            *data = FlashStore::flash_read_half_word(address);
        }
    }

    /// 定义参数存储模块保存数据到闪存函数
    pub fn store_save(&self) {
        // 擦除指定页
        self.flash_erase_page(STORE_START_ADDRESS);

        // 循环STORE_COUNT次，包括第一个标志位
        for (i, data) in unsafe { STORE_DATA.iter().enumerate() } {
            // 将SRAM数组的数据备份保存到闪存
            let address = STORE_START_ADDRESS + i as u32 * 2;
            self.flash_program_half_word(address, *data);
        }
    }

    /// 定义参数存储模块将所有有效数据清0函数
    pub fn store_clear(&self) {
        // 循环STORE_COUNT次，除了第一个标志位
        for data in unsafe { STORE_DATA[1..].iter_mut() } {
            // SRAM数组有效数据清0
            *data = 0x0000;
        }

        // 保存数据到闪存
        self.store_save();
    }
}
