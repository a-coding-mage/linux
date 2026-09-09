/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Atmel SFR (Special Function Registers) register offsets and bit definitions.
 *
 * Copyright (C) 2016 Atmel
 *
 * Author: Ludovic Desroches <ludovic.desroches@atmel.com>
 */

pub const AT91_SFR_DDRCFG: u32 = 0x04; /* DDR Configuration Register */
pub const AT91_SFR_CCFG_EBICSA: u32 = 0x04; /* EBI Chip Select Register */
/* 0x08 ~ 0x0c: Reserved */
pub const AT91_SFR_OHCIICR: u32 = 0x10; /* OHCI INT Configuration Register */
pub const AT91_SFR_OHCIISR: u32 = 0x14; /* OHCI INT Status Register */
pub const AT91_SFR_UTMICKTRIM: u32 = 0x30; /* UTMI Clock Trimming Register */
pub const AT91_SFR_UTMISWAP: u32 = 0x3c; /* UTMI DP/DM Pin Swapping Register */
pub const AT91_SFR_LS: u32 = 0x7c; /* Light Sleep Register */
pub const AT91_SFR_I2SCLKSEL: u32 = 0x90; /* I2SC Register */
pub const AT91_SFR_WPMR: u32 = 0xe4; /* Write Protection Mode Register */

/* Field definitions */
pub const fn AT91_SFR_CCFG_EBI_CSA(cs: u32, val: u32) -> u32 {
    val << cs
}
pub const AT91_SFR_CCFG_EBI_DBPUC: u32 = BIT(8);
pub const AT91_SFR_CCFG_EBI_DBPDC: u32 = BIT(9);
pub const AT91_SFR_CCFG_EBI_DRIVE: u32 = BIT(17);
pub const AT91_SFR_CCFG_NFD0_ON_D16: u32 = BIT(24);
pub const AT91_SFR_CCFG_DDR_MP_EN: u32 = BIT(25);

pub const fn AT91_SFR_OHCIICR_RES(x: u32) -> u32 {
    BIT(x)
}
pub const AT91_SFR_OHCIICR_ARIE: u32 = BIT(4);
pub const AT91_SFR_OHCIICR_APPSTART: u32 = BIT(5);
pub const fn AT91_SFR_OHCIICR_USB_SUSP(x: u32) -> u32 {
    BIT(8 + x)
}
pub const AT91_SFR_OHCIICR_UDPPUDIS: u32 = BIT(23);
pub const AT91_OHCIICR_USB_SUSPEND: u32 = GENMASK(10, 8);

pub const fn AT91_SFR_OHCIISR_RIS(x: u32) -> u32 {
    BIT(x)
}

pub const AT91_UTMICKTRIM_FREQ: u32 = GENMASK(1, 0);

pub const fn AT91_SFR_UTMISWAP_PORT(x: u32) -> u32 {
    BIT(x)
}

pub const fn AT91_SFR_LS_VALUE(x: u32) -> u32 {
    BIT(x)
}
pub const AT91_SFR_LS_MEM_POWER_GATING_ULP1_EN: u32 = BIT(16);

pub const AT91_SFR_WPMR_WPEN: u32 = BIT(0);
pub const AT91_SFR_WPMR_WPKEY_MASK: u32 = GENMASK(31, 8);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
