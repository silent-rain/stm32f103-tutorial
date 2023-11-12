//! 简单的 OLED 实例

use super::OLED;

use stm32f1xx_hal::gpio::{self, IOPinSpeed, OpenDrain, Output, OutputSpeed, PB8, PB9};

/// OLEDTY 对象别名
pub type OLEDTY = OLED<PB8<Output<OpenDrain>>, PB9<Output<OpenDrain>>>;

/// 初始化 OLED 显示屏引脚
/// pin: pb8、pb9
/// ```rust
/// use oled;
/// let mut oled = oled::simple::init_oled(gpiob.pb8, gpiob.pb9, &mut gpiob.crh);
/// oled.show_string(1, 1, "hallo");
/// ```
pub fn init_oled(
    pb8: PB8,
    pb9: PB9,
    crh: &mut gpio::Cr<'B', true>,
) -> OLED<PB8<Output<OpenDrain>>, PB9<Output<OpenDrain>>> {
    // 将引脚配置为作为开漏输出模式
    // scl（时钟线）：用于同步数据传输，控制数据的传输速度和顺序。
    // 在OLED显示屏中，scl 信号用于同步数据位的发送和接收。
    // sda（数据线）：用于传输数据到OLED显示屏。
    // 在OLED显示屏中，sda 信号用于传输每个像素的数据，包括颜色信息、亮度等。
    let mut scl = pb8.into_open_drain_output(crh);
    let mut sda = pb9.into_open_drain_output(crh);
    scl.set_speed(crh, IOPinSpeed::Mhz50);
    sda.set_speed(crh, IOPinSpeed::Mhz50);

    OLED::new(scl, sda)
}
