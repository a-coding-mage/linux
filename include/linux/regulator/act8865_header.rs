/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * act8865.h  --  Voltage regulation for active-semi act88xx PMUs
 *
 * Copyright (C) 2013 Atmel Corporation.
 */

// Dependency supplied by the Linux regulator machine interface:
// use linux::regulator::machine::{device_node, regulator_init_data};

pub const ACT8600_ID_DCDC1: i32 = 0;
pub const ACT8600_ID_DCDC2: i32 = 1;
pub const ACT8600_ID_DCDC3: i32 = 2;
pub const ACT8600_ID_SUDCDC4: i32 = 3;
pub const ACT8600_ID_LDO5: i32 = 4;
pub const ACT8600_ID_LDO6: i32 = 5;
pub const ACT8600_ID_LDO7: i32 = 6;
pub const ACT8600_ID_LDO8: i32 = 7;
pub const ACT8600_ID_LDO9: i32 = 8;
pub const ACT8600_ID_LDO10: i32 = 9;

pub const ACT8865_ID_DCDC1: i32 = 0;
pub const ACT8865_ID_DCDC2: i32 = 1;
pub const ACT8865_ID_DCDC3: i32 = 2;
pub const ACT8865_ID_LDO1: i32 = 3;
pub const ACT8865_ID_LDO2: i32 = 4;
pub const ACT8865_ID_LDO3: i32 = 5;
pub const ACT8865_ID_LDO4: i32 = 6;
pub const ACT8865_REG_NUM: i32 = 7;

pub const ACT8846_ID_REG1: i32 = 0;
pub const ACT8846_ID_REG2: i32 = 1;
pub const ACT8846_ID_REG3: i32 = 2;
pub const ACT8846_ID_REG4: i32 = 3;
pub const ACT8846_ID_REG5: i32 = 4;
pub const ACT8846_ID_REG6: i32 = 5;
pub const ACT8846_ID_REG7: i32 = 6;
pub const ACT8846_ID_REG8: i32 = 7;
pub const ACT8846_ID_REG9: i32 = 8;
pub const ACT8846_ID_REG10: i32 = 9;
pub const ACT8846_ID_REG11: i32 = 10;
pub const ACT8846_ID_REG12: i32 = 11;
pub const ACT8846_REG_NUM: i32 = 12;

pub const ACT8600: i32 = 0;
pub const ACT8865: i32 = 1;
pub const ACT8846: i32 = 2;

/**
 * act8865_regulator_data - regulator data
 * @id: regulator id
 * @name: regulator name
 * @init_data: regulator init data
 * @of_node: device tree node (optional)
 */
#[repr(C)]
pub struct act8865_regulator_data {
    pub id: i32,
    pub name: *const core::ffi::c_char,
    pub init_data: *mut regulator_init_data,
    pub of_node: *mut device_node,
}

/**
 * act8865_platform_data - platform data for act8865
 * @num_regulators: number of regulators used
 * @regulators: pointer to regulators used
 */
#[repr(C)]
pub struct act8865_platform_data {
    pub num_regulators: i32,
    pub regulators: *mut act8865_regulator_data,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
