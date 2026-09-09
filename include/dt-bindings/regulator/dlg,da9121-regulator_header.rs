/* SPDX-License-Identifier: GPL-2.0+ */

/*
 * These buck mode constants may be used to specify values in device tree
 * properties (e.g. regulator-initial-mode).
 * A description of the following modes is in the manufacturers datasheet.
 */

pub const DA9121_BUCK_MODE_FORCE_PFM: i32 = 0;
pub const DA9121_BUCK_MODE_FORCE_PWM: i32 = 1;
pub const DA9121_BUCK_MODE_FORCE_PWM_SHEDDING: i32 = 2;
pub const DA9121_BUCK_MODE_AUTO: i32 = 3;

pub const DA9121_BUCK_RIPPLE_CANCEL_NONE: i32 = 0;
pub const DA9121_BUCK_RIPPLE_CANCEL_SMALL: i32 = 1;
pub const DA9121_BUCK_RIPPLE_CANCEL_MID: i32 = 2;
pub const DA9121_BUCK_RIPPLE_CANCEL_LARGE: i32 = 3;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
