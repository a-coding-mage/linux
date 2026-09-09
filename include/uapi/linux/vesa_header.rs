/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/* VESA Blanking Levels */
#[allow(non_camel_case_types)]
pub type vesa_blank_mode = i32;

pub const VESA_NO_BLANKING: vesa_blank_mode = 0;
pub const VESA_VSYNC_SUSPEND: vesa_blank_mode = 1;
pub const VESA_HSYNC_SUSPEND: vesa_blank_mode = 2;
pub const VESA_POWERDOWN: vesa_blank_mode =
    VESA_VSYNC_SUSPEND | VESA_HSYNC_SUSPEND;
pub const VESA_BLANK_MAX: vesa_blank_mode = VESA_POWERDOWN;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
