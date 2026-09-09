/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2023 SiFive
 */

// Dependency supplied by the corresponding architecture switch-to header.
unsafe extern "C" {
    fn has_fpu() -> bool;
}

#[inline]
pub unsafe fn kernel_fpu_available() -> bool {
    has_fpu()
}

unsafe extern "C" {
    pub fn kernel_fpu_begin();
    pub fn kernel_fpu_end();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
