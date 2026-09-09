/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * TI TPS68470 PMIC platform data definition.
 *
 * Copyright (c) 2021 Red Hat Inc.
 *
 * Red Hat authors:
 * Hans de Goede <hdegoede@redhat.com>
 */

#[repr(u32)]
pub enum tps68470_regulators {
    TPS68470_CORE,
    TPS68470_ANA,
    TPS68470_VCM,
    TPS68470_VIO,
    TPS68470_VSIO,
    TPS68470_AUX1,
    TPS68470_AUX2,
    TPS68470_NUM_REGULATORS,
}

// Forward declaration of the externally defined regulator initialization data.
#[repr(C)]
pub struct regulator_init_data;

#[repr(C)]
pub struct tps68470_regulator_platform_data {
    pub reg_init_data: [*const regulator_init_data; TPS68470_NUM_REGULATORS as usize],
}

#[repr(C)]
pub struct tps68470_clk_consumer {
    pub consumer_dev_name: *const core::ffi::c_char,
    pub consumer_con_id: *const core::ffi::c_char,
}

#[repr(C)]
pub struct tps68470_clk_platform_data {
    pub n_consumers: u32,
    pub consumers: [tps68470_clk_consumer; 0],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
