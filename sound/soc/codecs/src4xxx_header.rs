// SPDX-License-Identifier: GPL-2.0
//
// src4xxx.h  --  SRC4XXX ALSA SoC audio driver
//
// Copyright 2021-2022 Deqx Pty Ltd
// Author: Matt R Flax <flatmax@flatmax.com>

use core::ffi::c_int;

#[repr(C)]
pub struct device {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct regmap {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct regmap_config {
    _unused: [u8; 0],
}

pub const SRC4XXX_RES_00: u32 = 0x00;
pub const SRC4XXX_PWR_RST_01: u32 = 0x01;
pub const SRC4XXX_RESET: u32 = 0x80;
pub const SRC4XXX_POWER_DOWN: u32 = 0x00;
pub const SRC4XXX_POWER_ENABLE: u32 = 0x20;
pub const SRC4XXX_ENABLE_SRC: u32 = 0x1;
pub const SRC4XXX_ENABLE_SRC_SHIFT: u32 = 0;
pub const SRC4XXX_ENABLE_DIR: u32 = 0x2;
pub const SRC4XXX_ENABLE_DIR_SHIFT: u32 = 1;
pub const SRC4XXX_ENABLE_DIT: u32 = 0x4;
pub const SRC4XXX_ENABLE_DIT_SHIFT: u32 = 2;
pub const SRC4XXX_ENABLE_PORT_B: u32 = 0x8;
pub const SRC4XXX_ENABLE_PORT_B_SHIFT: u32 = 3;
pub const SRC4XXX_ENABLE_PORT_A: u32 = 0x10;
pub const SRC4XXX_ENABLE_PORT_A_SHIFT: u32 = 4;

pub const SRC4XXX_PORTA_CTL_03: u32 = 0x03;
pub const SRC4XXX_BUS_MASTER: u32 = 0x8;
pub const SRC4XXX_BUS_LEFT_J: u32 = 0x0;
pub const SRC4XXX_BUS_I2S: u32 = 0x1;
pub const SRC4XXX_BUS_RIGHT_J_16: u32 = 0x4;
pub const SRC4XXX_BUS_RIGHT_J_18: u32 = 0x5;
pub const SRC4XXX_BUS_RIGHT_J_20: u32 = 0x6;
pub const SRC4XXX_BUS_RIGHT_J_24: u32 = 0x7;
pub const SRC4XXX_BUS_FMT_MS_MASK: u32 = 0xf;

pub const SRC4XXX_PORTA_CTL_04: u32 = 0x04;
pub const SRC4XXX_MCLK_DIV_MASK: u32 = 0x3;

pub const fn SRC4XXX_BUS_FMT(id: u32) -> u32 {
    SRC4XXX_PORTA_CTL_03 + 2 * id
}

pub const fn SRC4XXX_BUS_CLK(id: u32) -> u32 {
    SRC4XXX_PORTA_CTL_04 + 2 * id
}

pub const SRC4XXX_PORTB_CTL_05: u32 = 0x05;
pub const SRC4XXX_PORTB_CTL_06: u32 = 0x06;

pub const SRC4XXX_TX_CTL_07: u32 = 0x07;
pub const SRC4XXX_TX_MCLK_DIV_MASK: u32 = 0x60;
pub const SRC4XXX_TX_MCLK_DIV_SHIFT: u32 = 5;

pub const SRC4XXX_TX_CTL_08: u32 = 0x08;
pub const SRC4XXX_TX_CTL_09: u32 = 0x09;
pub const SRC4XXX_SRC_DIT_IRQ_MSK_0B: u32 = 0x0B;
pub const SRC4XXX_SRC_BTI_EN: u32 = 0x01;
pub const SRC4XXX_SRC_TSLIP_EN: u32 = 0x02;
pub const SRC4XXX_SRC_DIT_IRQ_MODE_0C: u32 = 0x0C;
pub const SRC4XXX_RCV_CTL_0D: u32 = 0x0D;
pub const SRC4XXX_RXCLK_RXCKI: u32 = 0x0;
pub const SRC4XXX_RXCLK_MCLK: u32 = 0x8;
pub const SRC4XXX_RCV_CTL_0E: u32 = 0x0E;
pub const SRC4XXX_REC_MCLK_EN: u32 = 0x1;
pub const SRC4XXX_PLL2_DIV_0: u32 = 0x0 << 1;
pub const SRC4XXX_PLL2_DIV_2: u32 = 0x1 << 1;
pub const SRC4XXX_PLL2_DIV_4: u32 = 0x2 << 1;
pub const SRC4XXX_PLL2_DIV_8: u32 = 0x3 << 1;
pub const SRC4XXX_PLL2_LOL: u32 = 0x8;
pub const SRC4XXX_RCV_PLL_0F: u32 = 0x0F;
pub const SRC4XXX_RCV_PLL_10: u32 = 0x10;
pub const SRC4XXX_RCV_PLL_11: u32 = 0x11;
pub const SRC4XXX_RVC_IRQ_MSK_16: u32 = 0x16;
pub const SRC4XXX_RVC_IRQ_MSK_17: u32 = 0x17;
pub const SRC4XXX_RVC_IRQ_MODE_18: u32 = 0x18;
pub const SRC4XXX_RVC_IRQ_MODE_19: u32 = 0x19;
pub const SRC4XXX_RVC_IRQ_MODE_1A: u32 = 0x1A;
pub const SRC4XXX_GPIO_1_1B: u32 = 0x1B;
pub const SRC4XXX_GPIO_2_1C: u32 = 0x1C;
pub const SRC4XXX_GPIO_3_1D: u32 = 0x1D;
pub const SRC4XXX_GPIO_4_1E: u32 = 0x1E;
pub const SRC4XXX_SCR_CTL_2D: u32 = 0x2D;
pub const SRC4XXX_SCR_CTL_2E: u32 = 0x2E;
pub const SRC4XXX_SCR_CTL_2F: u32 = 0x2F;
pub const SRC4XXX_SCR_CTL_30: u32 = 0x30;
pub const SRC4XXX_SCR_CTL_31: u32 = 0x31;
pub const SRC4XXX_PAGE_SEL_7F: u32 = 0x7F;

// read only registers
pub const SRC4XXX_GLOBAL_ITR_STS_02: u32 = 0x02;
pub const SRC4XXX_SRC_DIT_STS_0A: u32 = 0x0A;
pub const SRC4XXX_NON_AUDIO_D_12: u32 = 0x12;
pub const SRC4XXX_RVC_STS_13: u32 = 0x13;
pub const SRC4XXX_RVC_STS_14: u32 = 0x14;
pub const SRC4XXX_RVC_STS_15: u32 = 0x15;
pub const SRC4XXX_SUB_CODE_1F: u32 = 0x1F;
pub const SRC4XXX_SUB_CODE_20: u32 = 0x20;
pub const SRC4XXX_SUB_CODE_21: u32 = 0x21;
pub const SRC4XXX_SUB_CODE_22: u32 = 0x22;
pub const SRC4XXX_SUB_CODE_23: u32 = 0x23;
pub const SRC4XXX_SUB_CODE_24: u32 = 0x24;
pub const SRC4XXX_SUB_CODE_25: u32 = 0x25;
pub const SRC4XXX_SUB_CODE_26: u32 = 0x26;
pub const SRC4XXX_SUB_CODE_27: u32 = 0x27;
pub const SRC4XXX_SUB_CODE_28: u32 = 0x28;
pub const SRC4XXX_PC_PREAMBLE_HI_29: u32 = 0x29;
pub const SRC4XXX_PC_PREAMBLE_LO_2A: u32 = 0x2A;
pub const SRC4XXX_PD_PREAMBLE_HI_2B: u32 = 0x2B;
pub const SRC4XXX_PC_PREAMBLE_LO_2C: u32 = 0x2C;
pub const SRC4XXX_IO_RATIO_32: u32 = 0x32;
pub const SRC4XXX_IO_RATIO_33: u32 = 0x33;

unsafe extern "C" {
    pub fn src4xxx_probe(
        dev: *mut device,
        regmap: *mut regmap,
        switch_mode: Option<unsafe extern "C" fn(dev: *mut device)>,
    ) -> c_int;

    pub static src4xxx_regmap_config: regmap_config;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
