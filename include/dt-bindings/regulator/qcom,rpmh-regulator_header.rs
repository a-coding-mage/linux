/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (c) 2018, The Linux Foundation. All rights reserved. */

/*
 * These mode constants may be used to specify modes for various RPMh regulator
 * device tree properties (e.g. regulator-initial-mode). Each type of regulator
 * supports a subset of the possible modes.
 *
 * RPMH_REGULATOR_MODE_RET: Retention mode in which only an extremely small
 * load current is allowed. This mode is supported by LDO and SMPS type
 * regulators.
 * RPMH_REGULATOR_MODE_LPM: Low power mode in which a small load current is
 * allowed. This mode corresponds to PFM for SMPS and BOB type regulators.
 * This mode is supported by LDO, HFSMPS, BOB, and PMIC4 FTSMPS type
 * regulators.
 * RPMH_REGULATOR_MODE_AUTO: Auto mode in which the regulator hardware
 * automatically switches between LPM and HPM based upon the real-time load
 * current. This mode is supported by HFSMPS, BOB, and PMIC4 FTSMPS type
 * regulators.
 * RPMH_REGULATOR_MODE_HPM: High power mode in which the full rated current
 * of the regulator is allowed. This mode corresponds to PWM for SMPS and BOB
 * type regulators. This mode is supported by all types of regulators.
 */
pub const RPMH_REGULATOR_MODE_RET: i32 = 0;
pub const RPMH_REGULATOR_MODE_LPM: i32 = 1;
pub const RPMH_REGULATOR_MODE_AUTO: i32 = 2;
pub const RPMH_REGULATOR_MODE_HPM: i32 = 3;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
