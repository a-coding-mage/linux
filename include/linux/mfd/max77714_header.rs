/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Maxim MAX77714 Register and data structures definition.
 *
 * Copyright (C) 2022 Luca Ceresoli
 * Author: Luca Ceresoli <luca.ceresoli@bootlin.com>
 */

// Originally included <linux/bits.h> for BIT().

pub const MAX77714_INT_TOP: u32 = 0x00;
pub const MAX77714_INT_TOPM: u32 = 0x07; /* Datasheet says "read only", but it is RW */

pub const MAX77714_INT_TOP_ONOFF: u32 = 1u32 << 1;
pub const MAX77714_INT_TOP_RTC: u32 = 1u32 << 3;
pub const MAX77714_INT_TOP_GPIO: u32 = 1u32 << 4;
pub const MAX77714_INT_TOP_LDO: u32 = 1u32 << 5;
pub const MAX77714_INT_TOP_SD: u32 = 1u32 << 6;
pub const MAX77714_INT_TOP_GLBL: u32 = 1u32 << 7;

pub const MAX77714_32K_STATUS: u32 = 0x30;
pub const MAX77714_32K_STATUS_SIOSCOK: u32 = 1u32 << 5;
pub const MAX77714_32K_STATUS_XOSCOK: u32 = 1u32 << 4;
pub const MAX77714_32K_STATUS_32KSOURCE: u32 = 1u32 << 3;
pub const MAX77714_32K_STATUS_32KLOAD_MSK: u32 = 0x3;
pub const MAX77714_32K_STATUS_32KLOAD_SHF: u32 = 1;
pub const MAX77714_32K_STATUS_CRYSTAL_CFG: u32 = 1u32 << 0;

pub const MAX77714_32K_CONFIG: u32 = 0x31;
pub const MAX77714_32K_CONFIG_XOSC_RETRY: u32 = 1u32 << 4;

pub const MAX77714_CNFG_GLBL2: u32 = 0x91;
pub const MAX77714_WDTEN: u32 = 1u32 << 2;
pub const MAX77714_WDTSLPC: u32 = 1u32 << 3;
pub const MAX77714_TWD_MASK: u32 = 0x3;
pub const MAX77714_TWD_2s: u32 = 0x0;
pub const MAX77714_TWD_16s: u32 = 0x1;
pub const MAX77714_TWD_64s: u32 = 0x2;
pub const MAX77714_TWD_128s: u32 = 0x3;

pub const MAX77714_CNFG_GLBL3: u32 = 0x92;
pub const MAX77714_WDTC: u32 = 1u32 << 0;

pub const MAX77714_CNFG2_ONOFF: u32 = 0x94;
pub const MAX77714_WD_RST_WK: u32 = 1u32 << 5;

/* Interrupts */
#[repr(i32)]
pub enum Max77714Irq {
    MAX77714_IRQ_TOP_ONOFF,
    MAX77714_IRQ_TOP_RTC,  /* Real-time clock */
    MAX77714_IRQ_TOP_GPIO, /* GPIOs */
    MAX77714_IRQ_TOP_LDO,  /* Low-dropout regulators */
    MAX77714_IRQ_TOP_SD,   /* Step-down regulators */
    MAX77714_IRQ_TOP_GLBL, /* "Global resources": Low-Battery, overtemp... */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
