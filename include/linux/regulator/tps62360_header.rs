/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * tps62360.h -- TI tps62360
 *
 * Interface for regulator driver for TI TPS62360 Processor core supply
 *
 * Copyright (C) 2012 NVIDIA Corporation
 *
 * Author: Laxman Dewangan <ldewangan@nvidia.com>
 */

/* Dependency supplied by the surrounding regulator subsystem. */

/*
 * struct tps62360_regulator_platform_data - tps62360 regulator platform data.
 *
 * @reg_init_data: The regulator init data.
 * @en_discharge: Enable discharge the output capacitor via internal
 *                register.
 * @en_internal_pulldn: internal pull down enable or not.
 * @vsel0_def_state: Default state of vsel0. 1 if it is high else 0.
 * @vsel1_def_state: Default state of vsel1. 1 if it is high else 0.
 */
#[repr(C)]
pub struct tps62360_regulator_platform_data {
    pub reg_init_data: *mut crate::regulator_init_data,
    pub en_discharge: bool,
    pub en_internal_pulldn: bool,
    pub vsel0_def_state: core::ffi::c_int,
    pub vsel1_def_state: core::ffi::c_int,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
