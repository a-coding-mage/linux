/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * National Semiconductors LP3972 PMIC chip client interface
 *
 * Based on lp3971.h
 */

// Dependency intent from <linux/regulator/machine.h>:
// `regulator_init_data` is supplied by the surrounding translated code.

pub const LP3972_LDO1: i32 = 0;
pub const LP3972_LDO2: i32 = 1;
pub const LP3972_LDO3: i32 = 2;
pub const LP3972_LDO4: i32 = 3;
pub const LP3972_LDO5: i32 = 4;

pub const LP3972_DCDC1: i32 = 5;
pub const LP3972_DCDC2: i32 = 6;
pub const LP3972_DCDC3: i32 = 7;

pub const LP3972_NUM_REGULATORS: i32 = 8;

#[repr(C)]
pub struct lp3972_regulator_subdev {
    pub id: i32,
    pub initdata: *mut regulator_init_data,
}

#[repr(C)]
pub struct lp3972_platform_data {
    pub num_regulators: i32,
    pub regulators: *mut lp3972_regulator_subdev,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
