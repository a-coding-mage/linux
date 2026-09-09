/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * vDSO implementation for Hexagon
 *
 * Copyright (c) 2011, The Linux Foundation. All rights reserved.
 */

// Translated from the C header. `u32` is supplied by the surrounding dependencies.
#[repr(C)]
pub struct hexagon_vdso {
    pub rt_signal_trampoline: [u32; 2],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
