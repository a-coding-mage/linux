/* SPDX-License-Identifier: GPL-2.0 */

// _DT_BINDINGS_REGULATOR_MEDIATEK_MT6397_H_

/*
 * Buck mode constants which may be used in devicetree properties (eg.
 * regulator-initial-mode, regulator-allowed-modes).
 * See the manufacturer's datasheet for more information on these modes.
 */

pub const MT6397_BUCK_MODE_AUTO: u32 = 0;
pub const MT6397_BUCK_MODE_FORCE_PWM: u32 = 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
