/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2011 ST-Ericsson SA
 * Written on behalf of Linaro for ST-Ericsson
 *
 * Author: Linus Walleij <linus.walleij@linaro.org>
 */

// C dependencies: <linux/i2c.h>, <linux/regulator/machine.h>,
// <linux/regmap.h>, and <linux/bitops.h>.

/* These registers are accessed from more than one driver */
pub const STW_CONF1: u32 = 0x11;
pub const STW_CONF1_PDN_VMMC: u32 = 0x01;
pub const STW_CONF1_VMMC_MASK: u32 = 0x0e;
pub const STW_CONF1_VMMC_1_8V: u32 = 0x02;
pub const STW_CONF1_VMMC_2_85V: u32 = 0x04;
pub const STW_CONF1_VMMC_3V: u32 = 0x06;
pub const STW_CONF1_VMMC_1_85V: u32 = 0x08;
pub const STW_CONF1_VMMC_2_6V: u32 = 0x0a;
pub const STW_CONF1_VMMC_2_7V: u32 = 0x0c;
pub const STW_CONF1_VMMC_3_3V: u32 = 0x0e;
pub const STW_CONF1_MMC_LS_STATUS: u32 = 0x10;
pub const STW_PCTL_REG_LO: u32 = 0x1e;
pub const STW_PCTL_REG_HI: u32 = 0x1f;
pub const STW_CONF1_V_MONITORING: u32 = 0x20;
pub const STW_CONF1_IT_WARN: u32 = 0x40;
pub const STW_CONF1_PDN_VAUX: u32 = 0x80;
pub const STW_CONF2: u32 = 0x20;
pub const STW_CONF2_MASK_TWARN: u32 = 0x01;
pub const STW_CONF2_VMMC_EXT: u32 = 0x02;
pub const STW_CONF2_MASK_IT_WAKE_UP: u32 = 0x04;
pub const STW_CONF2_GPO1: u32 = 0x08;
pub const STW_CONF2_GPO2: u32 = 0x10;
pub const STW_VCORE_SLEEP: u32 = 0x21;

/**
 * struct stw481x - state holder for the Stw481x drivers
 * @i2c_client: corresponding I2C client
 * @map: regmap handle to access device registers
 */
#[repr(C)]
pub struct stw481x {
    pub client: *mut i2c_client,
    pub map: *mut regmap,
}

// Opaque types supplied by the Linux dependencies.
pub enum i2c_client {}
pub enum regmap {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
