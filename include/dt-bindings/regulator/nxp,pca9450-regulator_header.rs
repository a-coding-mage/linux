/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause) */
/*
 * Device Tree binding constants for the NXP PCA9450A/B/C PMIC regulators
 */

/*
 * Buck mode constants which may be used in devicetree properties (eg.
 * regulator-initial-mode, regulator-allowed-modes).
 * See the manufacturer's datasheet for more information on these modes.
 */

pub const PCA9450_BUCK_MODE_AUTO: i32 = 0;
pub const PCA9450_BUCK_MODE_FORCE_PWM: i32 = 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
