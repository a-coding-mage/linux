/* SPDX-License-Identifier: GPL-2.0 */

// Dependency equivalent of <linux/device.h> is supplied externally.

#[repr(C)]
pub struct omap_backlight_config {
    pub default_intensity: i32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
