/* SPDX-License-Identifier: GPL-2.0-only */

pub const CCCR: u32 = 0x0000; /* Core Clock Configuration Register */
pub const CCSR: u32 = 0x000C; /* Core Clock Status Register */
pub const CKEN: u32 = 0x0004; /* Clock Enable Register */
pub const OSCC: u32 = 0x0008; /* Oscillator Configuration Register */

pub const CCCR_N_MASK: u32 = 0x0380; /* Run Mode Frequency to Turbo Mode Frequency Multiplier */
pub const CCCR_M_MASK: u32 = 0x0060; /* Memory Frequency to Run Mode Frequency Multiplier */
pub const CCCR_L_MASK: u32 = 0x001f; /* Crystal Frequency to Memory Frequency Multiplier */

pub const CCCR_CPDIS_BIT: u32 = 31;
pub const CCCR_PPDIS_BIT: u32 = 30;
pub const CCCR_LCD_26_BIT: u32 = 27;
pub const CCCR_A_BIT: u32 = 25;

pub const CCSR_N2_MASK: u32 = CCCR_N_MASK;
pub const CCSR_M_MASK: u32 = CCCR_M_MASK;
pub const CCSR_L_MASK: u32 = CCCR_L_MASK;
pub const CCSR_N2_SHIFT: u32 = 7;

pub const CKEN_AC97CONF: u32 = 31; /* AC97 Controller Configuration */
pub const CKEN_CAMERA: u32 = 24; /* Camera Interface Clock Enable */
pub const CKEN_SSP1: u32 = 23; /* SSP1 Unit Clock Enable */
pub const CKEN_MEMC: u32 = 22; /* Memory Controller Clock Enable */
pub const CKEN_MEMSTK: u32 = 21; /* Memory Stick Host Controller */
pub const CKEN_IM: u32 = 20; /* Internal Memory Clock Enable */
pub const CKEN_KEYPAD: u32 = 19; /* Keypad Interface Clock Enable */
pub const CKEN_USIM: u32 = 18; /* USIM Unit Clock Enable */
pub const CKEN_MSL: u32 = 17; /* MSL Unit Clock Enable */
pub const CKEN_LCD: u32 = 16; /* LCD Unit Clock Enable */
pub const CKEN_PWRI2C: u32 = 15; /* PWR I2C Unit Clock Enable */
pub const CKEN_I2C: u32 = 14; /* I2C Unit Clock Enable */
pub const CKEN_FICP: u32 = 13; /* FICP Unit Clock Enable */
pub const CKEN_MMC: u32 = 12; /* MMC Unit Clock Enable */
pub const CKEN_USB: u32 = 11; /* USB Unit Clock Enable */
pub const CKEN_ASSP: u32 = 10; /* ASSP (SSP3) Clock Enable */
pub const CKEN_USBHOST: u32 = 10; /* USB Host Unit Clock Enable */
pub const CKEN_OSTIMER: u32 = 9; /* OS Timer Unit Clock Enable */
pub const CKEN_NSSP: u32 = 9; /* NSSP (SSP2) Clock Enable */
pub const CKEN_I2S: u32 = 8; /* I2S Unit Clock Enable */
pub const CKEN_BTUART: u32 = 7; /* BTUART Unit Clock Enable */
pub const CKEN_FFUART: u32 = 6; /* FFUART Unit Clock Enable */
pub const CKEN_STUART: u32 = 5; /* STUART Unit Clock Enable */
pub const CKEN_HWUART: u32 = 4; /* HWUART Unit Clock Enable */
pub const CKEN_SSP3: u32 = 4; /* SSP3 Unit Clock Enable */
pub const CKEN_SSP: u32 = 3; /* SSP Unit Clock Enable */
pub const CKEN_SSP2: u32 = 3; /* SSP2 Unit Clock Enable */
pub const CKEN_AC97: u32 = 2; /* AC97 Unit Clock Enable */
pub const CKEN_PWM1: u32 = 1; /* PWM1 Clock Enable */
pub const CKEN_PWM0: u32 = 0; /* PWM0 Clock Enable */

pub const OSCC_OON: u32 = 1 << 1; /* 32.768kHz OON (write-once only bit) */
pub const OSCC_OOK: u32 = 1 << 0; /* 32.768kHz OOK (read-only bit) */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
