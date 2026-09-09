/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2019 Western Digital Corporation or its affiliates.
 *
 * Authors:
 *	Atish Patra <atish.patra@wdc.com>
 */

// Dependency equivalent of: #include <linux/types.h>

extern "C" {
    pub fn riscv_cs_get_mult_shift(mult: *mut u32, shift: *mut u32);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
