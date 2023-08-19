//! 外设
use cortex_m::peripheral::NVIC;
use stm32f1xx_hal::flash::{self, FlashExt};
use stm32f1xx_hal::gpio::{gpioa, gpiob, GpioExt};
use stm32f1xx_hal::prelude::_stm32_hal_afio_AfioExt;
use stm32f1xx_hal::rcc::{self, RccExt};
use stm32f1xx_hal::timer::{SysDelay, SysTimerExt};
use stm32f1xx_hal::{afio, pac};

/// 外设
pub struct Peripheral {
    pub flash: flash::Parts,
    pub rcc: rcc::Rcc,
    pub syst: cortex_m::peripheral::SYST,
    pub afio: afio::Parts,
    pub exti: pac::EXTI,
    pub nvic: cortex_m::peripheral::NVIC,
    pub gpioa: gpioa::Parts,
    pub gpiob: gpiob::Parts,
}

impl Peripheral {
    /// 初始化外设
    pub fn new() -> Self {
        // 获取对外设的访问对象
        let cp = cortex_m::Peripherals::take().unwrap();
        let dp = pac::Peripherals::take().unwrap();

        let flash: flash::Parts = dp.FLASH.constrain();
        let rcc: rcc::Rcc = dp.RCC.constrain();
        let syst = cp.SYST;
        let afio: stm32f1xx_hal::afio::Parts = dp.AFIO.constrain();
        let exti: pac::EXTI = dp.EXTI;
        let nvic: NVIC = cp.NVIC;

        let gpioa: gpioa::Parts = dp.GPIOA.split();
        let gpiob: gpiob::Parts = dp.GPIOB.split();

        Self {
            flash,
            rcc,
            syst,
            afio,
            exti,
            nvic,
            gpioa,
            gpiob,
        }
    }

    /// 封装具有自定义精度的阻塞延迟函数
    pub fn sys_delay(
        mut flash: flash::Parts,
        rcc: rcc::Rcc,
        syst: cortex_m::peripheral::SYST,
    ) -> SysDelay {
        // 冻结系统中所有时钟的配置，并将冻结的频率存储在时钟中
        let clocks = rcc.cfgr.freeze(&mut flash.acr);

        // 具有自定义精度的阻塞延迟
        syst.delay(&clocks)
    }
}
