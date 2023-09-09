//! KEY 按键工具库

use embedded_hal::{digital::v2::InputPin, prelude::_embedded_hal_blocking_delay_DelayMs};
use stm32f1xx_hal::timer::SysDelay;

/// 获取按键的状态
/// 按键是否按下
pub fn get_key_status<Pin>(key1: &mut Pin, delay: &mut SysDelay) -> bool
where
    Pin: InputPin,
    <Pin as InputPin>::Error: core::fmt::Debug,
{
    if key1.is_low().unwrap() {
        // 按键按下抖动
        delay.delay_ms(20_u16);
        // 按着不动, 松手后跳出循环
        while key1.is_low().unwrap() {}
        // 按键松开抖动
        delay.delay_ms(20_u16);

        return true;
    }

    false
}
