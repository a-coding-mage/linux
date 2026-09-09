/* SPDX-License-Identifier: GPL-2.0 */

/*
 * BUCK/LDO mode constants which may be used in devicetree properties
 * (eg. regulator-allowed-modes).
 * See the manufacturer's datasheet for more information on these modes.
 */

pub const MT6360_OPMODE_LP: i32 = 2;
pub const MT6360_OPMODE_ULP: i32 = 3;
pub const MT6360_OPMODE_NORMAL: i32 = 0;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
