/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * tps51632-regulator.h -- TPS51632 regulator
 *
 * Interface for regulator driver for TPS51632 3-2-1 Phase D-Cap Step Down
 * Driverless Controller with serial VID control and DVFS.
 *
 * Copyright (C) 2012 NVIDIA Corporation

 * Author: Laxman Dewangan <ldewangan@nvidia.com>
 */

/* Dependency supplied by the surrounding regulator subsystem. */

/*
 * struct tps51632_regulator_platform_data - tps51632 regulator platform data.
 *
 * @reg_init_data: The regulator init data.
 * @enable_pwm_dvfs: Enable PWM DVFS or not.
 * @dvfs_step_20mV: Step for DVFS is 20mV or 10mV.
 * @max_voltage_uV: Maximum possible voltage in PWM-DVFS mode.
 * @base_voltage_uV: Base voltage when PWM-DVFS enabled.
 */
#[repr(C)]
pub struct tps51632_regulator_platform_data {
    pub reg_init_data: *mut regulator_init_data,
    pub enable_pwm_dvfs: bool,
    pub dvfs_step_20mV: bool,
    pub max_voltage_uV: i32,
    pub base_voltage_uV: i32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
