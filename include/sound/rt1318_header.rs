/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * linux/sound/rt1318.h -- Platform data for RT1318
 *
 * Copyright 2024 Realtek Semiconductor Corp.
 */

// C header guard: __LINUX_SND_RT1318_H

#[repr(C)]
pub struct rt1318_platform_data {
    pub init_r0_l: ::core::ffi::c_uint,
    pub init_r0_r: ::core::ffi::c_uint,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
