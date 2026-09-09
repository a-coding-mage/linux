/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Device Tree binding constants for the ACT8865 PMIC regulators
 */

/*
 * These constants should be used to specify regulator modes in device tree for
 * ACT8865 regulators as follows:
 * ACT8865_REGULATOR_MODE_FIXED: It is specific to DCDC regulators and it
 * specifies the usage of fixed-frequency PWM.
 *
 * ACT8865_REGULATOR_MODE_NORMAL: It is specific to LDO regulators and it
 * specifies the usage of normal mode.
 *
 * ACT8865_REGULATOR_MODE_LOWPOWER: For DCDC and LDO regulators; it specify
 * the usage of proprietary power-saving mode.
 */

pub const ACT8865_REGULATOR_MODE_FIXED: i32 = 1;
pub const ACT8865_REGULATOR_MODE_NORMAL: i32 = 2;
pub const ACT8865_REGULATOR_MODE_LOWPOWER: i32 = 3;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
