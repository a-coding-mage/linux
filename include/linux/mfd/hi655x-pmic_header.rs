/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Device driver for regulators in hi655x IC
 *
 * Copyright (c) 2016 HiSilicon Ltd.
 *
 * Authors:
 * Chen Feng <puck.chen@hisilicon.com>
 * Fei  Wang <w.f@huawei.com>
 */

// Dependency retained from the C header: <linux/gpio/consumer.h>

/* Hi655x registers are mapped to memory bus in 4 bytes stride */
pub const HI655X_STRIDE: u32 = 4;
pub const fn HI655X_BUS_ADDR(x: u32) -> u32 {
    x << 2
}

pub const HI655X_BITS: u32 = 8;

pub const HI655X_NR_IRQ: u32 = 32;

pub const HI655X_IRQ_STAT_BASE: u32 = 0x003 << 2;
pub const HI655X_IRQ_MASK_BASE: u32 = 0x007 << 2;
pub const HI655X_ANA_IRQM_BASE: u32 = 0x1b5 << 2;
pub const HI655X_IRQ_ARRAY: u32 = 4;
pub const HI655X_IRQ_MASK: u32 = 0xFF;
pub const HI655X_IRQ_CLR: u32 = 0xFF;
pub const HI655X_VER_REG: u32 = 0x00;

pub const PMU_VER_START: u32 = 0x10;
pub const PMU_VER_END: u32 = 0x38;

pub const RESERVE_INT: u32 = 7;
pub const PWRON_D20R_INT: u32 = 6;
pub const PWRON_D20F_INT: u32 = 5;
pub const PWRON_D4SR_INT: u32 = 4;
pub const VSYS_6P0_D200UR_INT: u32 = 3;
pub const VSYS_UV_D3R_INT: u32 = 2;
pub const VSYS_2P5_R_INT: u32 = 1;
pub const OTMP_D1R_INT: u32 = 0;

pub const RESERVE_INT_MASK: u32 = 1u32 << RESERVE_INT;
pub const PWRON_D20R_INT_MASK: u32 = 1u32 << PWRON_D20R_INT;
pub const PWRON_D20F_INT_MASK: u32 = 1u32 << PWRON_D20F_INT;
pub const PWRON_D4SR_INT_MASK: u32 = 1u32 << PWRON_D4SR_INT;
pub const VSYS_6P0_D200UR_INT_MASK: u32 = 1u32 << VSYS_6P0_D200UR_INT;
pub const VSYS_UV_D3R_INT_MASK: u32 = 1u32 << VSYS_UV_D3R_INT;
pub const VSYS_2P5_R_INT_MASK: u32 = 1u32 << VSYS_2P5_R_INT;
pub const OTMP_D1R_INT_MASK: u32 = 1u32 << OTMP_D1R_INT;

#[repr(C)]
pub struct hi655x_pmic {
    // struct device *dev;
    pub dev: *mut core::ffi::c_void,
    // struct regmap *regmap;
    pub regmap: *mut core::ffi::c_void,
    // struct gpio_desc *gpio;
    pub gpio: *mut core::ffi::c_void,
    pub ver: core::ffi::c_uint,
    // struct regmap_irq_chip_data *irq_data;
    pub irq_data: *mut core::ffi::c_void,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
