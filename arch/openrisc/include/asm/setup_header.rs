/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2021 Stafford Horne
 */

// C dependencies: <linux/init.h>, <asm-generic/setup.h>
// The declaration is excluded when compiling as assembler in the C header.

/// C `__init void or1k_early_setup(void *fdt);`
unsafe extern "C" {
    pub fn or1k_early_setup(fdt: *mut core::ffi::c_void);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
