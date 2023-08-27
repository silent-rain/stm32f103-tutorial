//! 外设
#![allow(unused)]

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
    pub tim2: pac::TIM2,
    pub tim3: pac::TIM3,
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
        let tim2: pac::TIM2 = dp.TIM2;
        let tim3: pac::TIM3 = dp.TIM3;
        let tim1: pac::TIM1 = dp.TIM1;
        let syst = cp.SYST;
        let afio: stm32f1xx_hal::afio::Parts = dp.AFIO.constrain();
        let exti: pac::EXTI = dp.EXTI;
        let nvic: NVIC = cp.NVIC;

        let gpioa: gpioa::Parts = dp.GPIOA.split();
        let gpiob: gpiob::Parts = dp.GPIOB.split();

        Self {
            flash,
            rcc,
            tim2,
            tim3,
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
        flash: &mut flash::Parts,
        clocks: &rcc::Clocks,
        syst: cortex_m::peripheral::SYST,
    ) -> SysDelay {
        // 具有自定义精度的阻塞延迟
        syst.delay(clocks)
    }
}
