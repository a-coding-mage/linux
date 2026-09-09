/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause) */
/*
 * Copyright (c) 2026 Arturia - All rights reserved.
 *
 * Device Tree binding constants for the FAN53555 PMIC regulator
 */

/*
 * Constants to specify regulator modes in device tree for SYR82X regulators
 * FAN53555_REGULATOR_MODE_FORCE_PWM: Force fixed PWM mode
 * FAN53555_REGULATOR_MODE_AUTO:      Allow auto-PFM mode during light load
 */

pub const FAN53555_REGULATOR_MODE_FORCE_PWM: u32 = 1;
pub const FAN53555_REGULATOR_MODE_AUTO: u32 = 2;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
