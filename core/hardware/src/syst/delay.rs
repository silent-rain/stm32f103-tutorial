//! 延时工具库

use stm32f1xx_hal::flash;
use stm32f1xx_hal::rcc;
use stm32f1xx_hal::timer::{SysDelay, SysTimerExt};

/// 封装具有自定义精度的阻塞延迟函数
pub fn sys_delay(
    flash: &mut flash::Parts,
    rcc: rcc::Rcc,
    syst: cortex_m::peripheral::SYST,
) -> SysDelay {
    // 冻结系统中所有时钟的配置，并将冻结的频率存储在时钟中
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    // 具有自定义精度的阻塞延迟
    syst.delay(&clocks)
}
