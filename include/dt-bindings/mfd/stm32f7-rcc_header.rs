/* SPDX-License-Identifier: GPL-2.0 */
/*
 * This header provides constants for the STM32F7 RCC IP
 */

/* AHB1 */
pub const STM32F7_RCC_AHB1_GPIOA: i32 = 0;
pub const STM32F7_RCC_AHB1_GPIOB: i32 = 1;
pub const STM32F7_RCC_AHB1_GPIOC: i32 = 2;
pub const STM32F7_RCC_AHB1_GPIOD: i32 = 3;
pub const STM32F7_RCC_AHB1_GPIOE: i32 = 4;
pub const STM32F7_RCC_AHB1_GPIOF: i32 = 5;
pub const STM32F7_RCC_AHB1_GPIOG: i32 = 6;
pub const STM32F7_RCC_AHB1_GPIOH: i32 = 7;
pub const STM32F7_RCC_AHB1_GPIOI: i32 = 8;
pub const STM32F7_RCC_AHB1_GPIOJ: i32 = 9;
pub const STM32F7_RCC_AHB1_GPIOK: i32 = 10;
pub const STM32F7_RCC_AHB1_CRC: i32 = 12;
pub const STM32F7_RCC_AHB1_BKPSRAM: i32 = 18;
pub const STM32F7_RCC_AHB1_DTCMRAM: i32 = 20;
pub const STM32F7_RCC_AHB1_DMA1: i32 = 21;
pub const STM32F7_RCC_AHB1_DMA2: i32 = 22;
pub const STM32F7_RCC_AHB1_DMA2D: i32 = 23;
pub const STM32F7_RCC_AHB1_ETHMAC: i32 = 25;
pub const STM32F7_RCC_AHB1_ETHMACTX: i32 = 26;
pub const STM32F7_RCC_AHB1_ETHMACRX: i32 = 27;
pub const STM32FF_RCC_AHB1_ETHMACPTP: i32 = 28;
pub const STM32F7_RCC_AHB1_OTGHS: i32 = 29;
pub const STM32F7_RCC_AHB1_OTGHSULPI: i32 = 30;

macro_rules! STM32F7_AHB1_RESET { ($bit:expr) => { $bit + (0x10 * 8) }; }
macro_rules! STM32F7_AHB1_CLOCK { ($bit:expr) => { $bit }; }

/* AHB2 */
pub const STM32F7_RCC_AHB2_DCMI: i32 = 0;
pub const STM32F7_RCC_AHB2_CRYP: i32 = 4;
pub const STM32F7_RCC_AHB2_HASH: i32 = 5;
pub const STM32F7_RCC_AHB2_RNG: i32 = 6;
pub const STM32F7_RCC_AHB2_OTGFS: i32 = 7;
macro_rules! STM32F7_AHB2_RESET { ($bit:expr) => { $bit + (0x14 * 8) }; }
macro_rules! STM32F7_AHB2_CLOCK { ($bit:expr) => { $bit + 0x20 }; }

/* AHB3 */
pub const STM32F7_RCC_AHB3_FMC: i32 = 0;
pub const STM32F7_RCC_AHB3_QSPI: i32 = 1;
macro_rules! STM32F7_AHB3_RESET { ($bit:expr) => { $bit + (0x18 * 8) }; }
macro_rules! STM32F7_AHB3_CLOCK { ($bit:expr) => { $bit + 0x40 }; }

/* APB1 */
pub const STM32F7_RCC_APB1_TIM2: i32 = 0;
pub const STM32F7_RCC_APB1_TIM3: i32 = 1;
pub const STM32F7_RCC_APB1_TIM4: i32 = 2;
pub const STM32F7_RCC_APB1_TIM5: i32 = 3;
pub const STM32F7_RCC_APB1_TIM6: i32 = 4;
pub const STM32F7_RCC_APB1_TIM7: i32 = 5;
pub const STM32F7_RCC_APB1_TIM12: i32 = 6;
pub const STM32F7_RCC_APB1_TIM13: i32 = 7;
pub const STM32F7_RCC_APB1_TIM14: i32 = 8;
pub const STM32F7_RCC_APB1_LPTIM1: i32 = 9;
pub const STM32F7_RCC_APB1_WWDG: i32 = 11;
pub const STM32F7_RCC_APB1_CAN3: i32 = 13;
pub const STM32F7_RCC_APB1_SPI2: i32 = 14;
pub const STM32F7_RCC_APB1_SPI3: i32 = 15;
pub const STM32F7_RCC_APB1_SPDIFRX: i32 = 16;
pub const STM32F7_RCC_APB1_UART2: i32 = 17;
pub const STM32F7_RCC_APB1_UART3: i32 = 18;
pub const STM32F7_RCC_APB1_UART4: i32 = 19;
pub const STM32F7_RCC_APB1_UART5: i32 = 20;
pub const STM32F7_RCC_APB1_I2C1: i32 = 21;
pub const STM32F7_RCC_APB1_I2C2: i32 = 22;
pub const STM32F7_RCC_APB1_I2C3: i32 = 23;
pub const STM32F7_RCC_APB1_I2C4: i32 = 24;
pub const STM32F7_RCC_APB1_CAN1: i32 = 25;
pub const STM32F7_RCC_APB1_CAN2: i32 = 26;
pub const STM32F7_RCC_APB1_CEC: i32 = 27;
pub const STM32F7_RCC_APB1_PWR: i32 = 28;
pub const STM32F7_RCC_APB1_DAC: i32 = 29;
pub const STM32F7_RCC_APB1_UART7: i32 = 30;
pub const STM32F7_RCC_APB1_UART8: i32 = 31;
macro_rules! STM32F7_APB1_RESET { ($bit:expr) => { $bit + (0x20 * 8) }; }
macro_rules! STM32F7_APB1_CLOCK { ($bit:expr) => { $bit + 0x80 }; }

/* APB2 */
pub const STM32F7_RCC_APB2_TIM1: i32 = 0;
pub const STM32F7_RCC_APB2_TIM8: i32 = 1;
pub const STM32F7_RCC_APB2_USART1: i32 = 4;
pub const STM32F7_RCC_APB2_USART6: i32 = 5;
pub const STM32F7_RCC_APB2_SDMMC2: i32 = 7;
pub const STM32F7_RCC_APB2_ADC1: i32 = 8;
pub const STM32F7_RCC_APB2_ADC2: i32 = 9;
pub const STM32F7_RCC_APB2_ADC3: i32 = 10;
pub const STM32F7_RCC_APB2_SDMMC1: i32 = 11;
pub const STM32F7_RCC_APB2_SPI1: i32 = 12;
pub const STM32F7_RCC_APB2_SPI4: i32 = 13;
pub const STM32F7_RCC_APB2_SYSCFG: i32 = 14;
pub const STM32F7_RCC_APB2_TIM9: i32 = 16;
pub const STM32F7_RCC_APB2_TIM10: i32 = 17;
pub const STM32F7_RCC_APB2_TIM11: i32 = 18;
pub const STM32F7_RCC_APB2_SPI5: i32 = 20;
pub const STM32F7_RCC_APB2_SPI6: i32 = 21;
pub const STM32F7_RCC_APB2_SAI1: i32 = 22;
pub const STM32F7_RCC_APB2_SAI2: i32 = 23;
pub const STM32F7_RCC_APB2_LTDC: i32 = 26;
pub const STM32F7_RCC_APB2_DSI: i32 = 27;
macro_rules! STM32F7_APB2_RESET { ($bit:expr) => { $bit + (0x24 * 8) }; }
macro_rules! STM32F7_APB2_CLOCK { ($bit:expr) => { $bit + 0xA0 }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
