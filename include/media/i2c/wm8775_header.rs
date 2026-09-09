/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
    wm8775.h - definition for wm8775 inputs and outputs

    Copyright (C) 2006 Hans Verkuil (hverkuil@kernel.org)

*/

/* The WM8775 has 4 inputs and one output. Zero or more inputs
   are multiplexed together to the output. Hence there are
   16 combinations.
   If only one input is active (the normal case) then the
   input values 1, 2, 4 or 8 should be used. */

pub const WM8775_AIN1: i32 = 1;
pub const WM8775_AIN2: i32 = 2;
pub const WM8775_AIN3: i32 = 4;
pub const WM8775_AIN4: i32 = 8;

#[repr(C)]
pub struct wm8775_platform_data {
    /*
     * FIXME: Instead, we should parameterize the params
     * that need different settings between ivtv, pvrusb2, and Nova-S
     */
    pub is_nova_s: bool,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
