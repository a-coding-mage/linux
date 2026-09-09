/*
 * This header provides constants for the STM32H7 RCC IP
 */

// C preprocessor token-pasting reset macros are represented as Rust macros
// taking the corresponding RCC bit value.

/* AHB3 */
pub const STM32H7_RCC_AHB3_MDMA: i32 = 0;
pub const STM32H7_RCC_AHB3_DMA2D: i32 = 4;
pub const STM32H7_RCC_AHB3_JPGDEC: i32 = 5;
pub const STM32H7_RCC_AHB3_FMC: i32 = 12;
pub const STM32H7_RCC_AHB3_QUADSPI: i32 = 14;
pub const STM32H7_RCC_AHB3_SDMMC1: i32 = 16;
pub const STM32H7_RCC_AHB3_CPU: i32 = 31;

macro_rules! STM32H7_AHB3_RESET { ($bit:expr) => { ($bit + (0x7C * 8)) }; }

/* AHB1 */
pub const STM32H7_RCC_AHB1_DMA1: i32 = 0;
pub const STM32H7_RCC_AHB1_DMA2: i32 = 1;
pub const STM32H7_RCC_AHB1_ADC12: i32 = 5;
pub const STM32H7_RCC_AHB1_ART: i32 = 14;
pub const STM32H7_RCC_AHB1_ETH1MAC: i32 = 15;
pub const STM32H7_RCC_AHB1_USB1OTG: i32 = 25;
pub const STM32H7_RCC_AHB1_USB2OTG: i32 = 27;

macro_rules! STM32H7_AHB1_RESET { ($bit:expr) => { ($bit + (0x80 * 8)) }; }

/* AHB2 */
pub const STM32H7_RCC_AHB2_CAMITF: i32 = 0;
pub const STM32H7_RCC_AHB2_CRYPT: i32 = 4;
pub const STM32H7_RCC_AHB2_HASH: i32 = 5;
pub const STM32H7_RCC_AHB2_RNG: i32 = 6;
pub const STM32H7_RCC_AHB2_SDMMC2: i32 = 9;

macro_rules! STM32H7_AHB2_RESET { ($bit:expr) => { ($bit + (0x84 * 8)) }; }

/* AHB4 */
pub const STM32H7_RCC_AHB4_GPIOA: i32 = 0;
pub const STM32H7_RCC_AHB4_GPIOB: i32 = 1;
pub const STM32H7_RCC_AHB4_GPIOC: i32 = 2;
pub const STM32H7_RCC_AHB4_GPIOD: i32 = 3;
pub const STM32H7_RCC_AHB4_GPIOE: i32 = 4;
pub const STM32H7_RCC_AHB4_GPIOF: i32 = 5;
pub const STM32H7_RCC_AHB4_GPIOG: i32 = 6;
pub const STM32H7_RCC_AHB4_GPIOH: i32 = 7;
pub const STM32H7_RCC_AHB4_GPIOI: i32 = 8;
pub const STM32H7_RCC_AHB4_GPIOJ: i32 = 9;
pub const STM32H7_RCC_AHB4_GPIOK: i32 = 10;
pub const STM32H7_RCC_AHB4_CRC: i32 = 19;
pub const STM32H7_RCC_AHB4_BDMA: i32 = 21;
pub const STM32H7_RCC_AHB4_ADC3: i32 = 24;
pub const STM32H7_RCC_AHB4_HSEM: i32 = 25;

macro_rules! STM32H7_AHB4_RESET { ($bit:expr) => { ($bit + (0x88 * 8)) }; }

/* APB3 */
pub const STM32H7_RCC_APB3_LTDC: i32 = 3;
pub const STM32H7_RCC_APB3_DSI: i32 = 4;
macro_rules! STM32H7_APB3_RESET { ($bit:expr) => { ($bit + (0x8C * 8)) }; }

/* APB1L */
pub const STM32H7_RCC_APB1L_TIM2: i32 = 0;
pub const STM32H7_RCC_APB1L_TIM3: i32 = 1;
pub const STM32H7_RCC_APB1L_TIM4: i32 = 2;
pub const STM32H7_RCC_APB1L_TIM5: i32 = 3;
pub const STM32H7_RCC_APB1L_TIM6: i32 = 4;
pub const STM32H7_RCC_APB1L_TIM7: i32 = 5;
pub const STM32H7_RCC_APB1L_TIM12: i32 = 6;
pub const STM32H7_RCC_APB1L_TIM13: i32 = 7;
pub const STM32H7_RCC_APB1L_TIM14: i32 = 8;
pub const STM32H7_RCC_APB1L_LPTIM1: i32 = 9;
pub const STM32H7_RCC_APB1L_SPI2: i32 = 14;
pub const STM32H7_RCC_APB1L_SPI3: i32 = 15;
pub const STM32H7_RCC_APB1L_SPDIF_RX: i32 = 16;
pub const STM32H7_RCC_APB1L_USART2: i32 = 17;
pub const STM32H7_RCC_APB1L_USART3: i32 = 18;
pub const STM32H7_RCC_APB1L_UART4: i32 = 19;
pub const STM32H7_RCC_APB1L_UART5: i32 = 20;
pub const STM32H7_RCC_APB1L_I2C1: i32 = 21;
pub const STM32H7_RCC_APB1L_I2C2: i32 = 22;
pub const STM32H7_RCC_APB1L_I2C3: i32 = 23;
pub const STM32H7_RCC_APB1L_HDMICEC: i32 = 27;
pub const STM32H7_RCC_APB1L_DAC12: i32 = 29;
pub const STM32H7_RCC_APB1L_USART7: i32 = 30;
pub const STM32H7_RCC_APB1L_USART8: i32 = 31;
macro_rules! STM32H7_APB1L_RESET { ($bit:expr) => { ($bit + (0x90 * 8)) }; }

/* APB1H */
pub const STM32H7_RCC_APB1H_CRS: i32 = 1;
pub const STM32H7_RCC_APB1H_SWP: i32 = 2;
pub const STM32H7_RCC_APB1H_OPAMP: i32 = 4;
pub const STM32H7_RCC_APB1H_MDIOS: i32 = 5;
pub const STM32H7_RCC_APB1H_FDCAN: i32 = 8;
macro_rules! STM32H7_APB1H_RESET { ($bit:expr) => { ($bit + (0x94 * 8)) }; }

/* APB2 */
pub const STM32H7_RCC_APB2_TIM1: i32 = 0;
pub const STM32H7_RCC_APB2_TIM8: i32 = 1;
pub const STM32H7_RCC_APB2_USART1: i32 = 4;
pub const STM32H7_RCC_APB2_USART6: i32 = 5;
pub const STM32H7_RCC_APB2_SPI1: i32 = 12;
pub const STM32H7_RCC_APB2_SPI4: i32 = 13;
pub const STM32H7_RCC_APB2_TIM15: i32 = 16;
pub const STM32H7_RCC_APB2_TIM16: i32 = 17;
pub const STM32H7_RCC_APB2_TIM17: i32 = 18;
pub const STM32H7_RCC_APB2_SPI5: i32 = 20;
pub const STM32H7_RCC_APB2_SAI1: i32 = 22;
pub const STM32H7_RCC_APB2_SAI2: i32 = 23;
pub const STM32H7_RCC_APB2_SAI3: i32 = 24;
pub const STM32H7_RCC_APB2_DFSDM1: i32 = 28;
pub const STM32H7_RCC_APB2_HRTIM: i32 = 29;
macro_rules! STM32H7_APB2_RESET { ($bit:expr) => { ($bit + (0x98 * 8)) }; }

/* APB4 */
pub const STM32H7_RCC_APB4_SYSCFG: i32 = 1;
pub const STM32H7_RCC_APB4_LPUART1: i32 = 3;
pub const STM32H7_RCC_APB4_SPI6: i32 = 5;
pub const STM32H7_RCC_APB4_I2C4: i32 = 7;
pub const STM32H7_RCC_APB4_LPTIM2: i32 = 9;
pub const STM32H7_RCC_APB4_LPTIM3: i32 = 10;
pub const STM32H7_RCC_APB4_LPTIM4: i32 = 11;
pub const STM32H7_RCC_APB4_LPTIM5: i32 = 12;
pub const STM32H7_RCC_APB4_COMP12: i32 = 14;
pub const STM32H7_RCC_APB4_VREF: i32 = 15;
pub const STM32H7_RCC_APB4_SAI4: i32 = 21;
pub const STM32H7_RCC_APB4_TMPSENS: i32 = 26;
macro_rules! STM32H7_APB4_RESET { ($bit:expr) => { ($bit + (0x9C * 8)) }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
