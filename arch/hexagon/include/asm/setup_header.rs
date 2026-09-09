/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2010-2011, The Linux Foundation. All rights reserved.
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License version 2 and
 * only version 2 as published by the Free Software Foundation.
 */

// C header guard: _ASM_HEXAGON_SETUP_H

// Corresponds to <linux/init.h> and <uapi/asm/setup.h>.

unsafe extern "C" {
    pub static mut external_cmdline_buffer: core::ffi::c_char;

    // C __init annotation is build-system/linker metadata; preserve its intent here.
    pub fn setup_arch_memory();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
