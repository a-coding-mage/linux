/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Author: Kamel Bouhra <kamel.bouhara@bootlin.com>
 */

pub const POWER_ON_REASON_REGULAR: &str = "regular power-up";
pub const POWER_ON_REASON_RTC: &str = "RTC wakeup";
pub const POWER_ON_REASON_WATCHDOG: &str = "watchdog timeout";
pub const POWER_ON_REASON_SOFTWARE: &str = "software reset";
pub const POWER_ON_REASON_RST_BTN: &str = "reset button action";
pub const POWER_ON_REASON_CPU_CLK_FAIL: &str = "CPU clock failure";
pub const POWER_ON_REASON_XTAL_FAIL: &str = "crystal oscillator failure";
pub const POWER_ON_REASON_BROWN_OUT: &str = "brown-out reset";
pub const POWER_ON_REASON_UNKNOWN: &str = "unknown reason";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
