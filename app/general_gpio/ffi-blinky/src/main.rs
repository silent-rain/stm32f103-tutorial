#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_probe as _;

use cortex_m_rt::entry;
use stm32f1xx_hal as _;

use stm32f10x_rs::{
    Delay_ms, FunctionalState, GPIOMode_TypeDef, GPIOSpeed_TypeDef, GPIO_Init, GPIO_InitTypeDef,
    GPIO_ResetBits, GPIO_SetBits, GPIO_TypeDef, RCC_APB2PeriphClockCmd,
};

// #define GPIO_Pin_0                 ((uint16_t)0x0001)  /*!< Pin 0 selected */
#[allow(non_upper_case_globals)]
const GPIO_Pin_0: u16 = 0x0001;

// #define RCC_APB2Periph_GPIOA             ((uint32_t)0x00000004)
#[allow(non_upper_case_globals)]
const RCC_APB2Periph_GPIOA: u32 = 0x00000004;

// #define GPIOA               ((GPIO_TypeDef *) GPIOA_BASE)
// #define GPIOA_BASE            (APB2PERIPH_BASE + 0x0800)
// #define APB2PERIPH_BASE       (PERIPH_BASE + 0x10000)
// #define PERIPH_BASE           ((uint32_t)0x40000000) /*!< Peripheral base address in the alias region */
#[allow(non_upper_case_globals)]
const GPIOA_BASE: u32 = 0x40000000 + 0x10000 + 0x0800;

#[entry]
fn main() -> ! {
    unsafe { RCC_APB2PeriphClockCmd(RCC_APB2Periph_GPIOA, FunctionalState::ENABLE) };

    #[allow(non_snake_case)]
    let mut GPIOA = unsafe { *(GPIOA_BASE as *const GPIO_TypeDef) };

    let mut gpio_init_structure = GPIO_InitTypeDef {
        GPIO_Mode: GPIOMode_TypeDef::GPIO_Mode_Out_PP,
        GPIO_Pin: GPIO_Pin_0,
        GPIO_Speed: GPIOSpeed_TypeDef::GPIO_Speed_50MHz,
    };
    unsafe { GPIO_Init(&mut GPIOA as *mut GPIO_TypeDef, &mut gpio_init_structure) };
    loop {
        unsafe {
            GPIO_ResetBits(&mut GPIOA as *mut GPIO_TypeDef, GPIO_Pin_0);
            Delay_ms(500);
            GPIO_SetBits(&mut GPIOA as *mut GPIO_TypeDef, GPIO_Pin_0);
            Delay_ms(500);
        };
    }
}
