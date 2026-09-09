/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * MFD core driver for the RT5033
 *
 * Copyright (C) 2014 Samsung Electronics
 * Author: Beomho Seo <beomho.seo@samsung.com>
 */

// C header dependencies:
// #include <linux/regulator/consumer.h>
// #include <linux/i2c.h>
// #include <linux/regmap.h>

/* RT5033 regulator IDs */
#[repr(C)]
pub enum rt5033_regulators {
    RT5033_BUCK = 0,
    RT5033_LDO,
    RT5033_SAFE_LDO,

    RT5033_REGULATOR_NUM,
}

#[repr(C)]
pub struct rt5033_dev {
    pub dev: *mut device,

    pub regmap: *mut regmap,
    pub irq_data: *mut regmap_irq_chip_data,
    pub irq: ::core::ffi::c_int,
    pub wakeup: bool,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
