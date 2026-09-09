/* SPDX-License-Identifier: GPL-2.0 */

// These buck mode constants may be used to specify values in device tree
// properties (e.g. regulator-initial-mode, regulator-allowed-modes).
// A description of the following modes is in the manufacturers datasheet.

pub const DA9211_BUCK_MODE_SLEEP: i32 = 1;
pub const DA9211_BUCK_MODE_SYNC: i32 = 2;
pub const DA9211_BUCK_MODE_AUTO: i32 = 3;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
