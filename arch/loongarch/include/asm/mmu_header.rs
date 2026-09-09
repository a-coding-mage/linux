/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2020-2022 Loongson Technology Corporation Limited
 */

// Dependencies from the original header: <linux/atomic.h>, <linux/spinlock.h>.

#[repr(C)]
pub struct mm_context_t {
    pub asid: [u64; NR_CPUS],
    pub vdso: *mut core::ffi::c_void,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
