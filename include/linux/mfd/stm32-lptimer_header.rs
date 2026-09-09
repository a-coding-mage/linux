/* SPDX-License-Identifier: GPL-2.0 */
/*
 * STM32 Low-Power Timer parent driver.
 * Copyright (C) STMicroelectronics 2017
 * Author: Fabrice Gasnier <fabrice.gasnier@st.com>
 * Inspired by Benjamin Gaignard's stm32-timers driver
 */

// Dependencies supplied by the surrounding kernel translation:
// `clk` and `regmap` are declared externally.

pub const STM32_LPTIM_ISR: u32 = 0x00; // Interrupt and Status Reg
pub const STM32_LPTIM_ICR: u32 = 0x04; // Interrupt Clear Reg
pub const STM32_LPTIM_IER: u32 = 0x08; // Interrupt Enable Reg
pub const STM32_LPTIM_CFGR: u32 = 0x0C; // Configuration Reg
pub const STM32_LPTIM_CR: u32 = 0x10; // Control Reg
pub const STM32_LPTIM_CMP: u32 = 0x14; // Compare Reg (MP25 CCR1)
pub const STM32_LPTIM_ARR: u32 = 0x18; // Autoreload Reg
pub const STM32_LPTIM_CNT: u32 = 0x1C; // Counter Reg
pub const STM32_LPTIM_CCMR1: u32 = 0x2C; // Capture/Compare Mode MP25
pub const STM32_LPTIM_CCR2: u32 = 0x34; // Compare Reg2 MP25

pub const STM32_LPTIM_HWCFGR2: u32 = 0x3EC; // Hardware configuration register 2 - MP25
pub const STM32_LPTIM_HWCFGR1: u32 = 0x3F0; // Hardware configuration register 1 - MP15
pub const STM32_LPTIM_VERR: u32 = 0x3F4; // Version identification register - MP15

// STM32_LPTIM_ISR - bit fields
pub const STM32_LPTIM_DIEROK_ARROK: u32 = (1u32 << 24) | (1u32 << 4); // MP25
pub const STM32_LPTIM_CMP2_ARROK: u32 = (1u32 << 19) | (1u32 << 4);
pub const STM32_LPTIM_CMPOK_ARROK: u32 = ((1u32 << (4 - 3 + 1)) - 1) << 3;
pub const STM32_LPTIM_ARROK: u32 = 1u32 << 4;
pub const STM32_LPTIM_CMPOK: u32 = 1u32 << 3;

// STM32_LPTIM_ICR - bit fields
pub const STM32_LPTIM_DIEROKCF_ARROKCF: u32 = (1u32 << 24) | (1u32 << 4); // MP25
pub const STM32_LPTIM_CMP2OKCF_ARROKCF: u32 = (1u32 << 19) | (1u32 << 4);
pub const STM32_LPTIM_CMPOKCF_ARROKCF: u32 = ((1u32 << (4 - 3 + 1)) - 1) << 3;
pub const STM32_LPTIM_ARRMCF: u32 = 1u32 << 1;

// STM32_LPTIM_IER - bit fields
pub const STM32_LPTIM_ARRMIE: u32 = 1u32 << 1;

// STM32_LPTIM_CR - bit fields
pub const STM32_LPTIM_CNTSTRT: u32 = 1u32 << 2;
pub const STM32_LPTIM_SNGSTRT: u32 = 1u32 << 1;
pub const STM32_LPTIM_ENABLE: u32 = 1u32 << 0;

// STM32_LPTIM_CFGR - bit fields
pub const STM32_LPTIM_ENC: u32 = 1u32 << 24;
pub const STM32_LPTIM_COUNTMODE: u32 = 1u32 << 23;
pub const STM32_LPTIM_WAVPOL: u32 = 1u32 << 21;
pub const STM32_LPTIM_PRESC: u32 = ((1u32 << (11 - 9 + 1)) - 1) << 9;
pub const STM32_LPTIM_CKPOL: u32 = ((1u32 << (2 - 1 + 1)) - 1) << 1;

// STM32_LPTIM_CKPOL
pub const STM32_LPTIM_CKPOL_RISING_EDGE: u32 = 0;
pub const STM32_LPTIM_CKPOL_FALLING_EDGE: u32 = 1;
pub const STM32_LPTIM_CKPOL_BOTH_EDGES: u32 = 2;

// STM32_LPTIM_ARR
pub const STM32_LPTIM_MAX_ARR: u32 = 0xFFFF;

// STM32_LPTIM_CCMR1
pub const STM32_LPTIM_CC2P: u32 = ((1u32 << (19 - 18 + 1)) - 1) << 18;
pub const STM32_LPTIM_CC2E: u32 = 1u32 << 17;
pub const STM32_LPTIM_CC2SEL: u32 = 1u32 << 16;
pub const STM32_LPTIM_CC1P: u32 = ((1u32 << (3 - 2 + 1)) - 1) << 2;
pub const STM32_LPTIM_CC1E: u32 = 1u32 << 1;
pub const STM32_LPTIM_CC1SEL: u32 = 1u32 << 0;

// STM32_LPTIM_HWCFGR1
pub const STM32_LPTIM_HWCFGR1_ENCODER: u32 = 1u32 << 16;

// STM32_LPTIM_HWCFGR2
pub const STM32_LPTIM_HWCFGR2_CHAN_NUM: u32 = (1u32 << (3 - 0 + 1)) - 1;

// STM32_LPTIM_VERR
pub const STM32_LPTIM_VERR_23: u32 = 0x23; // STM32MP25

/**
 * STM32 Low-Power Timer data assigned by parent device
 * @clk: clock reference for this instance
 * @regmap: register map reference for this instance
 * @has_encoder: indicates this Low-Power Timer supports encoder mode
 * @num_cc_chans: indicates the number of capture/compare channels
 * @version: indicates the major and minor revision of the controller
 */
#[repr(C)]
pub struct stm32_lptimer {
    pub clk: *mut clk,
    pub regmap: *mut regmap,
    pub has_encoder: bool,
    pub num_cc_chans: u32,
    pub version: u32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
